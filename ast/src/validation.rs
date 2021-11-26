use crate::{Expr, Root, Stmt};

pub fn validate(root: Root) {
  for stmt in root.stmts() {
    match stmt {
      Stmt::VariableDef(variable_def) => {
        if let Some(e) = variable_def.value() {
          validate_expr(e)
        }
      }
      Stmt::Expr(e) => validate_expr(e),
    }
  }
}

fn validate_expr(expr: Expr) {
  match expr {
    Expr::BinaryExpr(binary_expr) => {
      if let Some(e) = binary_expr.lhs() {
        validate_expr(e);
      }

      if let Some(e) = binary_expr.rhs() {
        validate_expr(e);
      }
    }
    Expr::Literal(literal) => {
      if literal.parse().is_none() {
        // report error
      }
    }
    Expr::ParenExpr(paren_expr) => {
      if let Some(e) = paren_expr.expr() {
        validate_expr(e);
      }
    }
    Expr::UnaryExpr(unary_expr) => {
      if let Some(e) = unary_expr.expr() {
        validate_expr(e);
      }
    }
    Expr::VariableRef(_) => {}
  }
}
