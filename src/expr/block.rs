use crate::stmt::Statement;
use crate::utils;

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
}

#[cfg(test)]
mod tests {
  use super::super::{BindingUsage, Expr, Number};
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
}
