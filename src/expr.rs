pub mod binding_usage;
pub mod block;

use crate::utils;
use crate::values::Value;
use binding_usage::BindingUsage;
use block::Block;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        let (number, remainder) = utils::extract_digits(s)?;
        Ok((Self(number.parse().unwrap()), remainder))
    }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        utils::tag("+", s)
            .map(|s| (Self::Add, s))
            .or_else(|_| utils::tag("-", s).map(|s| (Self::Sub, s)))
            .or_else(|_| utils::tag("*", s).map(|s| (Self::Mul, s)))
            .or_else(|_| utils::tag("/", s).map(|s| (Self::Div, s)))
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(Number),
    Operation {
        lhs: Number,
        rhs: Number,
        op: Operator,
    },
    BindingUsage(BindingUsage),
    Block(Block),
}

impl Expr {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        Self::new_operation(s)
            .or_else(|_| Self::new_number(s))
            .or_else(|_| {
                BindingUsage::new(s)
                    .map(|(binding_usage, s)| (Self::BindingUsage(binding_usage), s))
            })
            .or_else(|_| Block::new(s).map(|(block, s)| (Self::Block(block), s)))
    }

    fn new_operation(s: &str) -> Result<(Self, &str), String> {
        let (lhs, s) = Number::new(s)?;
        let (_, s) = utils::extract_whitespace(s);
        let (op, s) = Operator::new(s)?;
        let (_, s) = utils::extract_whitespace(s);

        let (rhs, s) = Number::new(s)?;

        Ok((Self::Operation { lhs, rhs, op }, s))
    }

    fn new_number(s: &str) -> Result<(Self, &str), String> {
        Number::new(s).map(|(number, s)| (Self::Number(number), s))
    }

    pub(crate) fn eval(&self) -> Value {
        match self {
            Self::Number(Number(n)) => Value::Number(*n),
            Self::Operation { lhs, rhs, op } => {
                let Number(lhs) = lhs;
                let Number(rhs) = rhs;

                let result = match op {
                    Operator::Add => lhs + rhs,
                    Operator::Sub => lhs - rhs,
                    Operator::Mul => lhs * rhs,
                    Operator::Div => lhs / rhs,
                };

                Value::Number(result)
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stmt::Statement;

    #[test]
    fn parse_expr_with_whitespace() {
        assert_eq!(
            Expr::new("2 * 2"),
            Ok((
                Expr::Operation {
                    lhs: Number(2),
                    rhs: Number(2),
                    op: Operator::Mul,
                },
                ""
            )),
        );
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expr::new("1+2"),
            Ok((
                Expr::Operation {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Operator::Add,
                },
                ""
            ))
        );
    }

    #[test]
    fn parse_add_operator() {
        assert_eq!(Operator::new("+"), Ok((Operator::Add, "")));
    }

    #[test]
    fn parse_sub_operator() {
        assert_eq!(Operator::new("-"), Ok((Operator::Sub, "")));
    }

    #[test]
    fn parse_mul_operator() {
        assert_eq!(Operator::new("*"), Ok((Operator::Mul, "")));
    }

    #[test]
    fn parse_div_operator() {
        assert_eq!(Operator::new("/"), Ok((Operator::Div, "")));
    }

    #[test]
    fn parse_numbers() {
        assert_eq!(Number::new("123"), Ok((Number(123), "")));
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(10),
                rhs: Number(10),
                op: Operator::Add,
            }
            .eval(),
            Value::Number(20),
        );
    }

    #[test]
    fn eval_sub() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(1),
                rhs: Number(5),
                op: Operator::Sub,
            }
            .eval(),
            Value::Number(-4),
        );
    }

    #[test]
    fn eval_mul() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(5),
                rhs: Number(6),
                op: Operator::Mul,
            }
            .eval(),
            Value::Number(30),
        );
    }

    #[test]
    fn eval_div() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(200),
                rhs: Number(20),
                op: Operator::Div,
            }
            .eval(),
            Value::Number(10),
        );
    }

    #[test]
    fn parse_number_as_expr() {
        assert_eq!(Expr::new("456"), Ok((Expr::Number(Number(456)), "")));
    }

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            Expr::new("bar"),
            Ok((
                Expr::BindingUsage(BindingUsage {
                    name: "bar".to_string(),
                }),
                "",
            )),
        );
    }

    #[test]
    fn parse_block() {
        assert_eq!(
            Expr::new("{ 200 }"),
            Ok((
                Expr::Block(Block {
                    statements: vec![Statement::Expr(Expr::Number(Number(200)))],
                }),
                "",
            )),
        );
    }
}
