use crate::lexer::TokenKind;
use num_traits::{FromPrimitive, ToPrimitive};

pub(crate) type SyntaxNode = rowan::SyntaxNode<MonkeLanguage>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum MonkeLanguage {}

impl rowan::Language for MonkeLanguage {
  type Kind = TokenKind;

  fn kind_from_raw(raw: rowan::TokenKind) -> Self::Kind {
    Self::Kind::from_u16(raw.0).unwrap()
  }

  fn kind_to_raw(kind: Self::Kind) -> rowan::TokenKind {
    rowan::TokenKind(kind.to_u16().unwrap())
  }
}
