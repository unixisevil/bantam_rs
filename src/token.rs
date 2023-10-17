use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Copy, Clone,  PartialEq, Eq, Hash)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Comma,
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,
    Tilde,
    Bang,
    Question,
    Colon,
    Name,
    Eof,
}

impl TokenType {
    pub fn punctuator(&self) -> Option<char> {
        match *self {
            Self::LeftParen => Some('('),
            Self::RightParen => Some(')'),
            Self::Comma => Some(','),
            Self::Assign => Some('='),
            Self::Plus => Some('+'),
            Self::Minus => Some('-'),
            Self::Asterisk => Some('*'),
            Self::Slash => Some('/'),
            Self::Caret => Some('^'),
            Self::Tilde => Some('~'),
            Self::Bang => Some('!'),
            Self::Question => Some('?'),
            Self::Colon => Some(':'),
            Self::Eof | Self::Name => None,
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Token<'source> {
    pub literal: &'source str,
    pub typ: TokenType,
}


