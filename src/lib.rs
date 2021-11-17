pub mod utils;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> (Self, &str) {
        let (number, remainder) = utils::extract_digits(s);
        (Self(number.parse().unwrap()), remainder)
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
    pub fn new(s: &str) -> (Self, &str) {
        let (op_str, remainder) = utils::extract_op(s);
        let op = match op_str {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => unreachable!(),
        };

        (op, remainder)
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Operator,
}

impl Expr {
    pub fn new(s: &str) -> (Self, &str) {
        let (_, s) = utils::extract_whitespace(s);
        let (lhs, s) = Number::new(s);
        let (_, s) = utils::extract_whitespace(s);
        let (op, s) = Operator::new(s);
        let (_, s) = utils::extract_whitespace(s);
        let (rhs, s) = Number::new(s);
        let (_, s) = utils::extract_whitespace(s);

        (Self { lhs, rhs, op }, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_expr_with_whitespace() {
        assert_eq!(
            Expr::new("2 * 2"),
            (
                Expr {
                    lhs: Number(2),
                    rhs: Number(2),
                    op: Operator::Mul,
                },
                ""
            ),
        );
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expr::new("1+2"),
            (
                Expr {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Operator::Add,
                },
                ""
            )
        );
    }

    #[test]
    fn parse_add_operator() {
        assert_eq!(Operator::new("+"), (Operator::Add, ""));
    }

    #[test]
    fn parse_sub_operator() {
        assert_eq!(Operator::new("-"), (Operator::Sub, ""));
    }

    #[test]
    fn parse_mul_operator() {
        assert_eq!(Operator::new("*"), (Operator::Mul, ""));
    }

    #[test]
    fn parse_div_operator() {
        assert_eq!(Operator::new("/"), (Operator::Div, ""));
    }

    #[test]
    fn parse_numbers() {
        assert_eq!(Number::new("123"), (Number(123), ""));
    }
}
