use crate::env::Env;
use crate::stmt::Statement;
use crate::utils;
use crate::values::Value;

#[derive(Debug, PartialEq)]
pub struct Block {
  pub statements: Vec<Statement>,
}

impl Block {
  pub fn new(s: &str) -> Result<(Self, &str), String> {
    let s = utils::tag("{", s)?;
    let (_, s) = utils::extract_whitespace(s);

    let mut s = s;
    let mut statments = Vec::new();

    while let Ok((statment, new_s)) = Statement::new(s) {
      statments.push(statment);

      let (_, whitespace_stripped) = utils::extract_whitespace(new_s);
      s = whitespace_stripped;
    }

    let (_, s) = utils::extract_whitespace(s);
    let s = utils::tag("}", s)?;

    Ok((
      Block {
        statements: statments,
      },
      s,
    ))
  }

  pub(crate) fn eval(&self, env: &Env) -> Result<Value, String> {
    if self.statements.is_empty() {
      return Ok(Value::Unit);
    }

    let mut child_env = env.create_child();

    let stmts_except_last = &self.statements[..self.statements.len() - 1];
    for stmt in stmts_except_last {
      stmt.eval(&mut child_env)?;
    }

    self.statements.last().unwrap().eval(&mut child_env)
  }
}

#[cfg(test)]
mod tests {
  use super::super::{BindingUsage, Expr, Number, Operator};
  use super::*;
  use crate::binding_def::BindingDef;

  #[test]
  fn parse_empty_block() {
    assert_eq!(
      Block::new("{}"),
      Ok((
        Block {
          statements: Vec::new()
        },
        ""
      ))
    );
  }

  #[test]
  fn parse_empty_block_with_whitespace() {
    assert_eq!(
      Block::new("{   }"),
      Ok((
        Block {
          statements: Vec::new()
        },
        ""
      ))
    );
  }

  #[test]
  fn parse_block_with_one_stmt_and_whitespace() {
    assert_eq!(
      Block::new("{ 5 }"),
      Ok((
        Block {
          statements: vec![Statement::Expr(Expr::Number(Number(5)))],
        },
        ""
      )),
    );
  }

  #[test]
  fn parse_block_with_one_stmt() {
    assert_eq!(
      Block::new("{5}"),
      Ok((
        Block {
          statements: vec![Statement::Expr(Expr::Number(Number(5)))],
        },
        ""
      ))
    )
  }

  #[test]
  fn parse_block_with_multiple_stmts() {
    assert_eq!(
      Block::new(
        "{
          let a = 10
          let b = a
          b
        }",
      ),
      Ok((
        Block {
          statements: vec![
            Statement::BindingDef(BindingDef {
              name: "a".to_string(),
              val: Expr::Number(Number(10)),
            }),
            Statement::BindingDef(BindingDef {
              name: "b".to_string(),
              val: Expr::BindingUsage(BindingUsage {
                name: "a".to_string()
              }),
            }),
            Statement::Expr(Expr::BindingUsage(BindingUsage {
              name: "b".to_string()
            })),
          ],
        },
        ""
      )),
    );
  }

  #[test]
  fn eval_empty_block() {
    assert_eq!(
      Block {
        statements: Vec::new()
      }
      .eval(&Env::default()),
      Ok(Value::Unit),
    );
  }

  #[test]
  fn eval_block_with_one_expr() {
    assert_eq!(
      Block {
        statements: vec![Statement::Expr(Expr::Number(Number(25)))],
      }
      .eval(&Env::default()),
      Ok(Value::Number(25)),
    );
  }

  #[test]
  fn eval_block_with_binding_def_and_usage() {
    assert_eq!(
      Block {
        statements: vec![
          Statement::BindingDef(BindingDef {
            name: "one".to_string(),
            val: Expr::Number(Number(1)),
          }),
          Statement::Expr(Expr::BindingUsage(BindingUsage {
            name: "one".to_string(),
          })),
        ],
      }
      .eval(&Env::default()),
      Ok(Value::Number(1)),
    );
  }

  #[test]
  fn eval_block_with_multiple_binding_defs() {
    assert_eq!(
      Block {
        statements: vec![
          Statement::BindingDef(BindingDef {
            name: "foo".to_string(),
            val: Expr::Number(Number(5)),
          }),
          Statement::BindingDef(BindingDef {
            name: "bar".to_string(),
            val: Expr::Number(Number(4)),
          }),
          Statement::BindingDef(BindingDef {
            name: "baz".to_string(),
            val: Expr::Number(Number(3)),
          }),
        ],
      }
      .eval(&Env::default()),
      Ok(Value::Unit),
    );
  }

  #[test]
  fn eval_block_with_multiple_exprs() {
    assert_eq!(
      Block {
        statements: vec![
          Statement::Expr(Expr::Number(Number(100))),
          Statement::Expr(Expr::Number(Number(30))),
          Statement::Expr(Expr::Operation {
            lhs: Number(10),
            rhs: Number(7),
            op: Operator::Sub,
          }),
        ],
      }
      .eval(&Env::default()),
      Ok(Value::Number(3)),
    );
  }

  #[test]
  fn eval_block_using_bindings_from_parent_env() {
    let mut env = Env::default();
    env.store_binding("foo".to_string(), Value::Number(2));

    assert_eq!(
      Block {
        statements: vec![
          Statement::BindingDef(BindingDef {
            name: "baz".to_string(),
            val: Expr::BindingUsage(BindingUsage {
              name: "foo".to_string(),
            }),
          }),
          Statement::Expr(Expr::BindingUsage(BindingUsage {
            name: "baz".to_string(),
          })),
        ],
      }
      .eval(&env),
      Ok(Value::Number(2)),
    );
  }
}
