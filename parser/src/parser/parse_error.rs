use std::fmt;
use syntax::SyntaxKind;
use text_size::TextRange;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct ParseError {
  pub(super) expected: Vec<SyntaxKind>,
  pub(super) found: Option<SyntaxKind>,
  pub(super) range: TextRange,
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "error at {}..{}: expected {}, but found {}",
      u32::from(self.range.start()),
      u32::from(self.range.end()),
      self.expected[0],
      self.found.unwrap(),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::ops::Range as StdRange;

  fn check(
    expected: Vec<SyntaxKind>,
    found: Option<SyntaxKind>,
    range: StdRange<u32>,
    output: &str,
  ) {
    let error = ParseError {
      expected,
      found,
      range: {
        let start = range.start.into();
        let end = range.end.into();
        TextRange::new(start, end)
      },
    };

    assert_eq!(format!("{}", error), output);
  }

  #[test]
  fn one_expected_did_find() {
    check(
      vec![SyntaxKind::Equals],
      Some(SyntaxKind::Identifier),
      10..20,
      "error at 10..20: expected ‘=’, but found identifier",
    );
  }
}
