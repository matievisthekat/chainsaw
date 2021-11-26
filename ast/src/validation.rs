use crate::{Expr, Root, Stmt};
use text_size::TextRange;

#[derive(Debug, PartialEq)]
pub struct ValidationError {
  message: String,
  range: TextRange,
}

pub fn validate(root: Root) -> Vec<ValidationError> {
  let mut errors = Vec::new();

  for stmt in root.stmts() {
    match stmt {
      Stmt::VariableDef(variable_def) => {
        if let Some(e) = variable_def.value() {
          validate_expr(e, &mut errors);
        }
      }
      Stmt::Expr(e) => validate_expr(e, &mut errors),
    }
  }

  errors
}

fn validate_expr(expr: Expr, errors: &mut Vec<ValidationError>) {
  match expr {
    Expr::BinaryExpr(binary_expr) => {
      if let Some(e) = binary_expr.lhs() {
        validate_expr(e, errors);
      }

      if let Some(e) = binary_expr.rhs() {
        validate_expr(e, errors);
      }
    }
    Expr::Literal(literal) => {
      if literal.parse().is_none() {
        errors.push(ValidationError {
          message: format!(
            "number literal is larger than an integer’s maximum value, {}",
            u64::MAX,
          ),
          range: literal.0.first_token().unwrap().text_range(),
        });
      }
    }
    Expr::ParenExpr(paren_expr) => {
      if let Some(e) = paren_expr.expr() {
        validate_expr(e, errors);
      }
    }
    Expr::UnaryExpr(unary_expr) => {
      if let Some(e) = unary_expr.expr() {
        validate_expr(e, errors);
      }
    }
    Expr::VariableRef(_) => {}
  }
}
