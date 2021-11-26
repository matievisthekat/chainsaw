use crate::lexer::{SyntaxKind, Token};

pub(super) struct Source<'t, 'input> {
  lexemes: &'t [Token<'input>],
  cursor: usize,
}

impl<'t, 'input> Source<'t, 'input> {
  pub(super) fn new(lexemes: &'t [Token<'input>]) -> Self {
    Self { lexemes, cursor: 0 }
  }

  pub(super) fn next_lexeme(&mut self) -> Option<&'t Token<'input>> {
    self.eat_trivia();

    let token = self.lexemes.get(self.cursor)?;
    self.cursor += 1;

    Some(token)
  }

  pub(super) fn peek_kind(&mut self) -> Option<SyntaxKind> {
    self.eat_trivia();
    self.peek_kind_raw()
  }

  fn eat_trivia(&mut self) {
    while self.at_trivia() {
      self.cursor += 1;
    }
  }

  fn at_trivia(&self) -> bool {
    self.peek_kind_raw().map_or(false, SyntaxKind::is_trivia)
  }

  fn peek_kind_raw(&self) -> Option<SyntaxKind> {
    self
      .lexemes
      .get(self.cursor)
      .map(|Token { kind, .. }| *kind)
  }
}
