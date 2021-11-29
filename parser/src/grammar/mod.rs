mod expr;
mod stmt;

use crate::parser::marker::CompletedMarker;
use crate::parser::Parser;
use lexer::TokenKind;
use syntax::SyntaxKind;

pub(crate) fn root(p: &mut Parser) -> CompletedMarker {
  let m = p.start();

  while !p.at_end() {
    stmt::stmt(p);
  }

  m.complete(p, SyntaxKind::Root)
}

#[cfg(test)]
mod tests {
  use crate::check;
  use expect_test::expect;

  #[test]
  fn parse_multiple_statements() {
    check(
      "set a = 1;\na",
      expect![[r#"
Root@0..12
  VariableDef@0..11
    SetKw@0..3 "set"
    Whitespace@3..4 " "
    Identifier@4..5 "a"
    Whitespace@5..6 " "
    Equals@6..7 "="
    Whitespace@7..8 " "
    Literal@8..9
      Number@8..9 "1"
    SemiColon@9..10 ";"
    Whitespace@10..11 "\n"
  VariableRef@11..12
    Identifier@11..12 "a""#]],
    );
  }
}
