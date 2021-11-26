use syntax::SyntaxKind;
use text_size::TextRange;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct ParseError {
  pub(super) expected: Vec<SyntaxKind>,
  pub(super) found: Option<SyntaxKind>,
  pub(super) range: TextRange,
}
