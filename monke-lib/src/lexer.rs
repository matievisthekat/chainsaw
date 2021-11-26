use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

pub(crate) struct Lexer<'a> {
  inner: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
  pub(crate) fn new(input: &'a str) -> Self {
    Self {
      inner: SyntaxKind::lexer(input),
    }
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = (SyntaxKind, &'a str);

  fn next(&mut self) -> Option<Self::Item> {
    let kind = self.inner.next()?;
    let text = self.inner.slice();

    Some((kind, text))
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Logos, FromPrimitive, ToPrimitive)]
pub(crate) enum SyntaxKind {
  Root,
  BinaryOp,

  #[token("func")]
  FuncKw,

  #[token("set")]
  SetKw,

  #[regex("[A-Za-z][A-Za-z0-9]+")]
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

  #[regex(" +")]
  Whitespace,

  #[error]
  Error,
}

#[cfg(test)]
mod tests {
  use super::*;

  fn check(input: &str, kind: SyntaxKind) {
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next(), Some((kind, input)));
  }

  #[test]
  fn lex_spaces() {
    check("   ", SyntaxKind::Whitespace);
  }

  #[test]
  fn lex_func_keyword() {
    check("func", SyntaxKind::FuncKw);
  }

  #[test]
  fn lex_set_keyword() {
    check("set", SyntaxKind::SetKw);
  }

  #[test]
  fn lex_alphabetic_identifier() {
    check("foo", SyntaxKind::Identifier);
  }

  #[test]
  fn lex_alphanumeric_identifier() {
    check("foo123", SyntaxKind::Identifier);
  }

  #[test]
  fn lex_aphabetic_mixed_case_identifier() {
    check("FOObarBaZ", SyntaxKind::Identifier);
  }

  #[test]
  fn lex_alphanumeric_mixed_case_identifier() {
    check("fooBARbAz123", SyntaxKind::Identifier);
  }

  #[test]
  fn lex_number() {
    check("123", SyntaxKind::Number);
  }

  #[test]
  fn lex_plus() {
    check("+", SyntaxKind::Plus);
  }

  #[test]
  fn lex_minus() {
    check("-", SyntaxKind::Minus);
  }

  #[test]
  fn lex_asterisk() {
    check("*", SyntaxKind::Asterisk);
  }

  #[test]
  fn lex_slash() {
    check("/", SyntaxKind::Slash);
  }

  #[test]
  fn lex_equals() {
    check("=", SyntaxKind::Equals);
  }

  #[test]
  fn lex_lbrace() {
    check("{", SyntaxKind::LBrace);
  }

  #[test]
  fn lex_rbrace() {
    check("}", SyntaxKind::RBrace);
  }
}
