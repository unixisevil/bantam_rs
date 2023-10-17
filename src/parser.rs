use crate::expression::Expr;
use crate::lexer::Lexer;
pub use crate::parselets::*;
use crate::token::{Token, TokenType};
use rustc_hash::FxHashMap;

type PrefixMap<'callback> = FxHashMap<TokenType, &'callback dyn PrefixParselet>;
type InfixMap<'callback> = FxHashMap<TokenType, &'callback dyn InfixParselet>;

pub struct Parser<'callback, 'source> {
    prefix_map: PrefixMap<'callback>,
    infix_map: InfixMap<'callback>,
    tokbuf: Vec<Token<'source>>,
    lexer: Lexer<'source>,
}

impl<'source: 'callback, 'callback> Parser<'callback, 'source> {
    pub fn new(lexer: Lexer<'source>) -> Self {
        Self {
            prefix_map: PrefixMap::default(),
            infix_map: InfixMap::default(),
            tokbuf: Vec::new(),
            lexer,
        }
    }

    pub fn register_prefix(&mut self, tt: TokenType, prefix: &'callback dyn PrefixParselet) {
        self.prefix_map.insert(tt, prefix);
    }

    pub fn register_infix(&mut self, tt: TokenType, infix: &'callback dyn InfixParselet) {
        self.infix_map.insert(tt, infix);
    }

    pub fn parse_expression(&mut self) -> Box<Expr<'source>> {
        self.parse_expression_prec(0)
    }

    pub fn parse_expression_prec(&mut self, prec: u8) -> Box<Expr<'source>> {
        let mut tok = self.consume();
        let Some(prefix) = self.prefix_map.get(&tok.typ) else {
            panic!("Could not parse {:?} ", tok);
        };
        let mut left = prefix.parse(self, tok);
        while prec < self.lookahead_precedence() {
            tok = self.consume();
            let infix = self.infix_map[&tok.typ];
            left = infix.parse(self, left, tok);
        }
        left
    }

    fn lookahead_precedence(&mut self) -> u8 {
        let tt = self.look_ahead(0).typ;
        if let Some(infix_parselet) = self.infix_map.get(&tt) {
            infix_parselet.precedence()
        } else {
            0
        }
    }

    pub fn match_type(&mut self, expected: TokenType) -> bool {
        if self.look_ahead(0).typ != expected {
            false
        } else {
            self.consume();
            true
        }
    }

    pub fn consume_type(&mut self, expected: TokenType) -> Token<'source> {
        let tt = self.look_ahead(0).typ;
        if tt != expected {
            panic!(
                "{}",
                format!("Expected token type {:?}, but found {:?}", expected, tt)
            );
        }
        self.consume()
    }

    pub fn consume(&mut self) -> Token<'source> {
        self.look_ahead(0);
        self.tokbuf.remove(0)
    }

    fn look_ahead(&mut self, distance: usize) -> Token<'source> {
        while self.tokbuf.len() <= distance {
            let tok = self.lexer.next_token();
            self.tokbuf.push(tok);
        }
        self.tokbuf[distance]
    }
}

//pub struct TokenError(String);
//pub struct ParseError(String);

pub struct BantamParser(Parser<'static, 'static>);

impl BantamParser {
    pub fn new(source: &'static str) -> Self {
        let l = Lexer::new(source);
        let mut p = Self(Parser::new(l));

        static NAME: Name = Name;
        p.0.register_prefix(TokenType::Name, &NAME);

        static ASSIGN: Assign = Assign;
        p.0.register_infix(TokenType::Assign, &ASSIGN);

        static QUESTION: Cond = Cond;
        p.0.register_infix(TokenType::Question, &QUESTION);

        static GROUP: Group = Group;
        p.0.register_prefix(TokenType::LeftParen, &GROUP);

        static CALL: Call = Call;
        p.0.register_infix(TokenType::LeftParen, &CALL);

        static PRE: UnaryPrefix = UnaryPrefix {
            prec: crate::Precedence::Prefix as u8,
        };
        p.0.register_prefix(TokenType::Plus, &PRE);
        p.0.register_prefix(TokenType::Minus, &PRE);
        p.0.register_prefix(TokenType::Tilde, &PRE);
        p.0.register_prefix(TokenType::Bang, &PRE);

        static POST: UnaryPostfix = UnaryPostfix {
            prec: crate::Precedence::Postfix as u8,
        };
        p.0.register_infix(TokenType::Bang, &POST);

        static SUM: Binary = Binary {
            prec: crate::Precedence::Sum as u8,
            right: false,
        };
        static PRODUCT: Binary = Binary {
            prec: crate::Precedence::Product as u8,
            right: false,
        };
        p.0.register_infix(TokenType::Plus, &SUM);
        p.0.register_infix(TokenType::Minus, &SUM);
        p.0.register_infix(TokenType::Asterisk, &PRODUCT);
        p.0.register_infix(TokenType::Slash, &PRODUCT);

        static EXP: Binary = Binary {
            prec: crate::Precedence::Exponent as u8,
            right: true,
        };
        p.0.register_infix(TokenType::Caret, &EXP);

        p
    }

    pub fn parse_expression(&mut self) -> Box<Expr> {
        self.0.parse_expression()
    }
}

