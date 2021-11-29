use lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

pub type SyntaxNode = rowan::SyntaxNode<MonkeLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<MonkeLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<MonkeLanguage>;

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum SyntaxKind {
  Whitespace,
  FuncKw,
  SetKw,
  SemiColon,
  Identifier,
  Number,
  String,
  Plus,
  Minus,
  Asterisk,
  Slash,
  Equals,
  LParen,
  RParen,
  LBrace,
  RBrace,
  Comment,
  Error,
  Root,
  InfixExpr,
  ParenExpr,
  PrefixExpr,
  VariableRef,
  VariableDef,
}

impl From<TokenKind> for SyntaxKind {
  fn from(token_kind: TokenKind) -> Self {
    match token_kind {
      TokenKind::Whitespace => Self::Whitespace,
      TokenKind::FuncKw => Self::FuncKw,
      TokenKind::SetKw => Self::SetKw,
      TokenKind::SemiColon => Self::SemiColon,
      TokenKind::Identifier => Self::Identifier,
      TokenKind::Number => Self::Number,
      TokenKind::String => Self::String,
      TokenKind::Plus => Self::Plus,
      TokenKind::Minus => Self::Minus,
      TokenKind::Asterisk => Self::Asterisk,
      TokenKind::Slash => Self::Slash,
      TokenKind::Equals => Self::Equals,
      TokenKind::LParen => Self::LParen,
      TokenKind::RParen => Self::RParen,
      TokenKind::LBrace => Self::LBrace,
      TokenKind::RBrace => Self::RBrace,
      TokenKind::Comment => Self::Comment,
      TokenKind::Error => Self::Error,
    }
  }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MonkeLanguage {}

impl rowan::Language for MonkeLanguage {
  type Kind = SyntaxKind;

  fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
    Self::Kind::from_u16(raw.0).unwrap()
  }

  fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
    rowan::SyntaxKind(kind.to_u16().unwrap())
  }
}
