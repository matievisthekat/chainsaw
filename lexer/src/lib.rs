use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

pub(crate) struct Lexer<'a> {
  inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
  pub(crate) fn new(input: &'a str) -> Self {
    Self {
      inner: TokenKind::lexer(input),
    }
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    let kind = self.inner.next()?;
    let text = self.inner.slice();

    Some(Self::Item { kind, text })
  }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Token<'a> {
  pub(crate) kind: TokenKind,
  pub(crate) text: &'a str,
}

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
pub(crate) enum TokenKind {
  #[token("func")]
  FuncKw,

  #[token("set")]
  SetKw,

  #[regex("[A-Za-z][A-Za-z0-9]*")]
  Identifier,

  #[regex("[0-9]+")]
  Number,

  #[token("+")]
  Plus,

  #[token("-")]
  Minus,

  #[token("*")]
  Asterisk,

  #[token("/")]
  Slash,

  #[token("=")]
  Equals,

  #[token("{")]
  LBrace,

  #[token("}")]
  RBrace,

  #[token("(")]
  LParen,

  #[token(")")]
  RParen,

  #[regex("[ \n]+")]
  Whitespace,

  #[regex("#.*")]
  Comment,

  #[error]
  Error,
}

#[cfg(test)]
mod tests {
  use super::*;

  fn check(input: &str, kind: TokenKind) {
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next(), Some(Token { kind, text: input }));
  }

  #[test]
  fn lex_spaces() {
    check("   ", TokenKind::Whitespace);
  }

  #[test]
  fn lex_spaces_and_newlines() {
    check("  \n ", TokenKind::Whitespace);
  }

  #[test]
  fn lex_func_keyword() {
    check("func", TokenKind::FuncKw);
  }

  #[test]
  fn lex_set_keyword() {
    check("set", TokenKind::SetKw);
  }

  #[test]
  fn lex_single_char_identifier() {
    check("x", TokenKind::Identifier);
  }

  #[test]
  fn lex_alphabetic_identifier() {
    check("foo", TokenKind::Identifier);
  }

  #[test]
  fn lex_alphanumeric_identifier() {
    check("foo123", TokenKind::Identifier);
  }

  #[test]
  fn lex_aphabetic_mixed_case_identifier() {
    check("FOObarBaZ", TokenKind::Identifier);
  }

  #[test]
  fn lex_alphanumeric_mixed_case_identifier() {
    check("fooBARbAz123", TokenKind::Identifier);
  }

  #[test]
  fn lex_number() {
    check("123", TokenKind::Number);
  }

  #[test]
  fn lex_plus() {
    check("+", TokenKind::Plus);
  }

  #[test]
  fn lex_minus() {
    check("-", TokenKind::Minus);
  }

  #[test]
  fn lex_asterisk() {
    check("*", TokenKind::Asterisk);
  }

  #[test]
  fn lex_slash() {
    check("/", TokenKind::Slash);
  }

  #[test]
  fn lex_equals() {
    check("=", TokenKind::Equals);
  }

  #[test]
  fn lex_lbrace() {
    check("{", TokenKind::LBrace);
  }

  #[test]
  fn lex_lparen() {
    check("(", TokenKind::LParen);
  }

  #[test]
  fn lex_rparen() {
    check(")", TokenKind::RParen);
  }

  #[test]
  fn lex_rbrace() {
    check("}", TokenKind::RBrace);
  }

  #[test]
  fn lex_comment() {
    check("# foo", TokenKind::Comment);
  }
}
