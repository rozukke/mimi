use crate::span::{Offset, Span};

use crate::lexer::cursor::Cursor;
use regex::Regex;

mod cursor;

/// Lexed token.
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

/// Type information for a lexed token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Ident,
    Literal { kind: LiteralKind },
    Semic,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Whitespace,
    Unknown,
    Eof,
}

/// Type information for a token literal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Int,
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind != TokenKind::Eof {
            Some(token)
        } else {
            None
        }
    })
}

impl Cursor<'_> {
    pub fn advance_token(&mut self) -> Token {
        if self.is_eof() {
            return Token {
                kind: TokenKind::Eof,
                span: Span::default(),
            };
        }

        // TODO: Switch to a more performant charstream impl
        // Add new constructs here. Not sure how to do this in an exhaustive way, but that would be nice.
        let patterns = vec![
            (TokenKind::Ident, Regex::new(r"^[a-zA-Z_]\w*\b").unwrap()),
            (
                TokenKind::Literal {
                    kind: LiteralKind::Int,
                },
                Regex::new(r"^[0-9]+\b").unwrap(),
            ),
            (TokenKind::Semic, Regex::new(r"^;").unwrap()),
            (TokenKind::OpenParen, Regex::new(r"^\(").unwrap()),
            (TokenKind::CloseParen, Regex::new(r"^\)").unwrap()),
            (TokenKind::OpenBrace, Regex::new(r"^\{").unwrap()),
            (TokenKind::CloseBrace, Regex::new(r"^\}").unwrap()),
            (TokenKind::Whitespace, Regex::new(r"^\s+").unwrap()),
        ];

        for (kind, re) in patterns {
            if let Some(tok) = re.find(self.at_curr_pt()) {
                // Only check starting exactly at cursor position
                let token = Token {
                    kind,
                    span: Span::new(
                        Offset(self.curr_pt() as u32),
                        Offset((self.curr_pt() + tok.len()) as u32),
                    ),
                };
                self.advance(tok.len());
                return token;
            }
        }

        self.bump();
        Token {
            kind: TokenKind::Unknown,
            span: Span::new(
                Offset((self.curr_pt() - 1) as u32),
                Offset(self.curr_pt() as u32),
            ),
        }
    }
}
