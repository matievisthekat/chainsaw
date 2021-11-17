use crate::environment::Environment;
use crate::expr::Expr;
use crate::utils;

#[derive(Debug, PartialEq)]
pub struct Binding {
  name: String,
  val: Expr,
}

impl Binding {
  pub fn new(s: &str) -> Result<(Self, &str), String> {
    let s = utils::tag("let", s)?;
    let (_, s) = utils::extract_whitespace_required(s)?;

    let (name, s) = utils::extract_ident(s)?;
    let (_, s) = utils::extract_whitespace(s);

    let s = utils::tag("=", s)?;
    let (_, s) = utils::extract_whitespace(s);

    let (val, s) = Expr::new(s)?;

    Ok((
      Self {
        name: name.to_string(),
        val,
      },
      s,
    ))
  }

  pub(crate) fn eval(&self, env: &mut Environment) {
    env.store_binding(self.name.clone(), self.val.eval());
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::expr::{Number, Operator};

  #[test]
  fn cannot_parse_binding_without_space_after_let() {
    assert_eq!(
      Binding::new("letaaa=1+2"),
      Err("expected a space".to_string()),
    );
  }

  #[test]
  fn parse_binding_def() {
    assert_eq!(
      Binding::new("let a = 10 / 2"),
      Ok((
        Binding {
          name: "a".to_string(),
          val: Expr::Operation {
            lhs: Number(10),
            rhs: Number(2),
            op: Operator::Div,
          },
        },
        "",
      )),
    );
  }
}
