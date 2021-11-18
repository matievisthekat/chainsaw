use crate::binding_def::BindingDef;
use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub enum Statement {
  BindingDef(BindingDef),
  Expr(Expr),
}

impl Statement {
  pub fn new(s: &str) -> Result<(Self, &str), String> {
    BindingDef::new(s)
      .map(|(binding_def, s)| (Self::BindingDef(binding_def), s))
      .or_else(|_| Expr::new(s).map(|(expr, s)| (Self::Expr(expr), s)))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::expr::{Number, Operator};

  #[test]
  fn parse_binding_def() {
    assert_eq!(
      Statement::new("let a = 10"),
      Ok((
        Statement::BindingDef(BindingDef {
          name: "a".to_string(),
          val: Expr::Number(Number(10)),
        }),
        ""
      )),
    );
  }

  #[test]
  fn parse_expr() {
    assert_eq!(
      Statement::new("1+1"),
      Ok((
        Statement::Expr(Expr::Operation {
          lhs: Number(1),
          rhs: Number(1),
          op: Operator::Add,
        }),
        ""
      )),
    );
  }
}
