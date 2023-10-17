use crate::token::*;
use core::str::CharIndices;
use rustc_hash::FxHashMap;
use strum::IntoEnumIterator;

pub struct Lexer<'source> {
    punctuators: FxHashMap<char, TokenType>,
    input: &'source str,
    iter: CharIndices<'source>,
    c: char,
    ci: usize,
}

impl<'source> Lexer<'source> {
    pub fn new(input: &'source str) -> Self {
        let mut punctuators = FxHashMap::<char, TokenType>::default();
        for tt in TokenType::iter() {
            if let Some(punc) = tt.punctuator() {
                punctuators.insert(punc, tt);
            }
        }

        let mut lex = Self {
            iter: input.char_indices(),
            c: '\x00',
            ci: 0,
            input,
            punctuators,
        };

        lex.scan_char();
        lex
    }

    pub fn next_token(&mut self) -> Token<'source> {
        loop {
            self.skip_chars();
            if self.is_at_end() {
                break Token {
                    literal: "",
                    typ: TokenType::Eof,
                };
            }
            if let Some(&tt) = self.punctuators.get(&self.c) {
                let tok = Token {
                    typ: tt,
                    literal: &self.input[self.ci..self.ci + 1],
                };
                self.scan_char();
                break tok;
            } else if self.c.is_alphabetic() {
                break self.scan_name();
            } else {
                self.scan_char();
            }
        }
    }

    fn scan_name(&mut self) -> Token<'source> {
        let start = self.ci;
        while self.c.is_alphabetic() {
            self.scan_char();
        }
        Token {
            literal: &self.input[start..self.ci],
            typ: TokenType::Name,
        }
    }

    fn scan_char(&mut self) {
        if let Some((index, chr)) = self.iter.next() {
            self.ci = index;
            self.c = chr;
        } else {
            self.ci = self.input.len();
            self.c = '\x00';
        }
    }

    fn skip_chars(&mut self) {
        while self.c == ' ' || self.c == '\t' || self.c == '\r' || self.c == '\n' {
            self.scan_char()
        }
    }

    fn is_at_end(&self) -> bool {
        self.ci >= self.input.len()
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Token<'source>;
    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.next_token();
        if tok.typ == TokenType::Eof {
            None
        } else {
            Some(tok)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_tok {
        ($tok:expr, $wantype:expr, $wantval:expr) => {
            assert_eq!(
                $tok,
                Token {
                    typ: $wantype,
                    literal: $wantval
                }
            );
        };
        ($tok:expr, $wantype:expr) => {
            let tok = $tok;
            assert_eq!(
                $tok,
                Token {
                    typ: $wantype,
                    ..tok
                }
            );
        };
    }

    #[test]
    fn normal_case() {
        let lex = Lexer::new("a=b+c");
        let toks: Vec<Token> = lex.collect();
        /*
        for t in &toks {
            println!("{:?}", t);
        }
        */
        assert_eq!(toks.len(), 5);
        assert_tok!(toks[0], TokenType::Name, "a");
        assert_tok!(toks[1], TokenType::Assign);
    }

    #[test]
    fn skip_blank() {
        let lex = Lexer::new(" a = \t b    + \n c ");
        let toks: Vec<Token> = lex.collect();
        assert_eq!(toks.len(), 5);
        assert_tok!(toks[0], TokenType::Name, "a");
        assert_tok!(toks[2], TokenType::Name, "b");
        assert_tok!(toks[4], TokenType::Name, "c");
    }

    #[test]
    fn name_contain_chinese() {
        let lex = Lexer::new(" 我们 =  我 + 们 ");
        let toks: Vec<Token> = lex.collect();
        assert_eq!(toks.len(), 5);
        assert_tok!(toks[0], TokenType::Name, "我们");
        assert_tok!(toks[2], TokenType::Name, "我");
        assert_tok!(toks[4], TokenType::Name, "们");
    }

    #[test]
    fn ignore_other_chars() {
        let lex = Lexer::new("c123 = a[] + bb{}f");
        let toks: Vec<Token> = lex.collect();
        assert_eq!(toks.len(), 6);
        assert_tok!(toks[0], TokenType::Name, "c");
        assert_tok!(toks[2], TokenType::Name, "a");
        assert_tok!(toks[4], TokenType::Name, "bb");
        assert_tok!(toks[5], TokenType::Name, "f");
    }

    #[test]
    fn multi_operator_expr() {
        let lex = Lexer::new("a = b + c * d ^ e - f / g");
        let toks: Vec<Token> = lex.collect();
        assert_eq!(toks.len(), 13);
        assert_tok!(toks[10], TokenType::Name, "f");
        assert_tok!(toks[5], TokenType::Asterisk);
        assert_tok!(toks[7], TokenType::Caret);
        assert_tok!(toks[12], TokenType::Name, "g");
    }

    #[test]
    fn return_multi_eof() {
        let mut lex = Lexer::new("a=b+c");
        let mut toks: Vec<Token> = Vec::with_capacity(16);
        for _ in 0..16 {
            toks.push(lex.next_token());
        }
        assert_eq!(toks.len(), 16);
        toks.iter().skip(5).all(|tok| tok == &Token{typ: TokenType::Eof, literal: ""});
    }
}
