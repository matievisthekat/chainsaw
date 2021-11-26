use super::*;

pub(super) fn stmt(p: &mut Parser) -> Option<CompletedMarker> {
  if p.at(TokenKind::SetKw) {
    Some(variable_def(p))
  } else {
    expr::expr(p)
  }
}

fn variable_def(p: &mut Parser) -> CompletedMarker {
  assert!(p.at(TokenKind::SetKw));
  let m = p.start();
  p.bump();

  p.expect(TokenKind::Identifier);
  p.expect(TokenKind::Equals);

  expr::expr(p);

  m.complete(p, SyntaxKind::VariableDef)
}

#[cfg(test)]
mod tests {
  use crate::check;
  use expect_test::expect;

  #[test]
  fn parse_variable_definition() {
    check(
      "set foo = bar",
      expect![[r#"
Root@0..13
  VariableDef@0..13
    SetKw@0..3 "set"
    Whitespace@3..4 " "
    Identifier@4..7 "foo"
    Whitespace@7..8 " "
    Equals@8..9 "="
    Whitespace@9..10 " "
    VariableRef@10..13
      Identifier@10..13 "bar""#]],
    );
  }

  #[test]
  fn recover_on_let_token() {
    check(
      "set a =\nset b = a",
      expect![[r#"
          Root@0..17
            VariableDef@0..8
              SetKw@0..3 "set"
              Whitespace@3..4 " "
              Identifier@4..5 "a"
              Whitespace@5..6 " "
              Equals@6..7 "="
              Whitespace@7..8 "\n"
            VariableDef@8..17
              SetKw@8..11 "set"
              Whitespace@11..12 " "
              Identifier@12..13 "b"
              Whitespace@13..14 " "
              Equals@14..15 "="
              Whitespace@15..16 " "
              VariableRef@16..17
                Identifier@16..17 "a"
          error at 8..11: expected number, identifier, '-' or '(', but found 'set'"#]],
    );
  }
}
