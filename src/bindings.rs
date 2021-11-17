use crate::expr::Expr;
use crate::utils;

#[derive(Debug, PartialEq)]
pub struct Binding {
  name: String,
  val: Expr,
}

impl Binding {
  pub fn new(s: &str) -> (Self, &str) {
    let s = utils::tag("let", s);
    let (_, s) = utils::extract_whitespace(s);

    let (name, s) = utils::extract_ident(s);
    let (_, s) = utils::extract_whitespace(s);

    let s = utils::tag("=", s);
    let (_, s) = utils::extract_whitespace(s);

    let (val, s) = Expr::new(s);

    (
      Self {
        name: name.to_string(),
        val,
      },
      s,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::expr::{Number, Operator};

  #[test]
  fn parse_binding_def() {
    assert_eq!(
      Binding::new("let a = 10 / 2"),
      (
        Binding {
          name: "a".to_string(),
          val: Expr {
            lhs: Number(10),
            rhs: Number(2),
            op: Operator::Div,
          },
        },
        "",
      ),
    );
  }
}
