use crate::expression::Expr;
use crate::parser::Parser;
use crate::token::{Token,TokenType};

pub trait InfixParselet {
    fn parse<'source: 'callback, 'callback: 'parser, 'parser>(
        &'callback self,
        parser: &'parser mut Parser<'callback, 'source>,
        left: Box<Expr<'source>>,
        token: Token,
    ) -> Box<Expr<'source>>;

    fn precedence(&self) -> u8;
}

pub trait PrefixParselet {
    fn parse<'source: 'callback, 'callback: 'parser, 'parser>(
        &'callback self,
        parser: &'parser mut Parser<'callback, 'source>,
        token: Token<'source>,
    ) -> Box<Expr<'source>>;
}

pub struct Assign;

impl InfixParselet for Assign {
    fn parse<'s:'c, 'c: 'p, 'p>(
        &'c self,
        parser: &'p mut Parser<'c, 's>,
        left: Box<Expr<'s>>,
        _token: Token,
    ) -> Box<Expr<'s>> {
        let right = parser.parse_expression_prec(self.precedence() - 1);
        let Expr::Name(name) = *left else {
            panic!("The left-hand side of an assignment must be a name.");
        };

        Box::new(Expr::Assign { name, right })
    }

    fn precedence(&self) -> u8 {
        crate::Precedence::Assignment as u8
    }
}

pub struct Cond;

impl InfixParselet for Cond {
    fn parse<'s:'c, 'c: 'p, 'p>(
        &'c self,
        parser: &'p mut Parser<'c, 's>,
        left: Box<Expr<'s>>,
        _token: Token,
    ) -> Box<Expr<'s>> {
        let then_arm = parser.parse_expression();
        parser.consume_type(TokenType::Colon);
        let else_arm = parser.parse_expression_prec(self.precedence()-1);

        Box::new(Expr::Cond { cond: left, then_arm, else_arm})
    }

    fn precedence(&self) -> u8 {
        crate::Precedence::Conditional as u8
    }
}

pub struct Binary{
    pub prec: u8,
    pub right: bool,
}

impl InfixParselet for Binary {
    fn parse<'s:'c, 'c: 'p, 'p>(
        &'c self,
        parser: &'p mut Parser<'c, 's>,
        left: Box<Expr<'s>>,
        token: Token,
    ) -> Box<Expr<'s>> {
        let right =  parser.parse_expression_prec(self.prec - if self.right {1} else {0});
        Box::new(Expr::Infix{ left, op: token.typ, right})
    }

    fn precedence(&self) -> u8 {
        self.prec
    }
}

#[derive(Copy, Clone)]
pub  struct UnaryPrefix{ pub prec: u8 }

impl PrefixParselet for UnaryPrefix {
    fn parse<'source: 'callback, 'callback: 'parser, 'parser>(
        &'callback self,
        parser: &'parser mut Parser<'callback, 'source>,
        token: Token,
    ) -> Box<Expr<'source>> {
        let right = parser.parse_expression_prec(self.prec);
        Box::new(Expr::Prefix { op: token.typ, right })
    }
}

pub struct Group;

impl PrefixParselet for Group {
    fn parse<'source: 'callback, 'callback: 'parser, 'parser>(
        &'callback self,
        parser: &'parser mut Parser<'callback, 'source>,
        _token: Token,
    ) -> Box<Expr<'source>> {
        let expr = parser.parse_expression();
        parser.consume_type(TokenType::RightParen);
        expr
    }
}

pub struct Name;

impl PrefixParselet for Name {
    fn parse<'source: 'callback, 'callback: 'parser, 'parser>(
        &'callback self,
        _parser: &'parser mut Parser<'callback, 'source>,
        token: Token<'source>,
    ) -> Box<Expr<'source>> {
        Box::new(Expr::Name(token.literal))
    }
}



pub struct UnaryPostfix{ pub prec: u8 }

impl InfixParselet for UnaryPostfix {
    fn parse<'s:'c, 'c: 'p, 'p>(
        &'c self,
        _parser: &'p mut Parser<'c, 's>,
        left: Box<Expr<'s>>,
        token: Token,
    ) -> Box<Expr<'s>> {
        Box::new(Expr::Postfix { left, op: token.typ})
    }

    fn precedence(&self) -> u8 {
        self.prec
    }
}

pub struct Call;

impl InfixParselet for Call {
    fn parse<'s:'c, 'c: 'p, 'p>(
        &'c self,
        parser: &'p mut Parser<'c, 's>,
        left: Box<Expr<'s>>,
        _token: Token,
    ) -> Box<Expr<'s>> {
        let mut args: Vec<Box<Expr>> = vec![];
        if !parser.match_type(TokenType::RightParen) {
            loop {
                args.push(parser.parse_expression());
                if !parser.match_type(TokenType::Comma){
                    break;
                }
            }
            parser.consume_type(TokenType::RightParen);
        }
        Box::new(Expr::Call {func: left, args})
    }

    fn precedence(&self) -> u8 {
        crate::Precedence::Call as u8
    }
}



