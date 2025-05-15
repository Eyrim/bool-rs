use crate::logic_gates::{Gate, LogicGate};

pub trait EvaluateBooleanExpression {
    fn evaluate(&self) -> bool;
}

#[derive(PartialEq, Eq, Debug)]
pub struct BooleanOperation<'a> {
    operation_symbol: Option<&'a str>,
}

impl BooleanOperation<'_> {
    pub fn gate(&self) -> Gate {
        Gate::try_from(self.operation_symbol.unwrap()).unwrap()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct BooleanExpression<'a> {
    pub raw: &'a str,
    pub members: Vec<i8>,
    pub operation: BooleanOperation<'a>,
}

impl EvaluateBooleanExpression for BooleanExpression<'_> {
    fn evaluate(&self) -> bool {
        let gate = self.operation.gate();

        //TODO: Remove clone
        gate.test(self.members.clone())
    }
}

pub fn evaluate_expression(s: &str) -> bool {
    let without_whitespace = remove_spaces_from(s);

    let (_, expr) = expression_parsing::boolean_expression(&without_whitespace).unwrap();

    expr.evaluate()
}

fn remove_spaces_from(s: &str) -> String {
    s.chars()
        .filter(|c| !c.eq(&' '))
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
pub mod parsing_tests {
    use super::*;

    #[test]
    fn should_remove_spaces_from_string_that_contains_whitespace() {
        let input = "( a + c)";
        let expected = "(a+c)";

        let actual = remove_spaces_from(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_not_remove_spaces_from_string_with_no_whitespace() {
        let input = "(a+c)";
        let expected = input;

        let actual = remove_spaces_from(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_do_nothing_to_empty_string() {
        let input = "";
        let expected = input;

        let actual = remove_spaces_from(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_not_remove_newline_characters_when_no_spaces() {
        let input = "a+\nc";
        let expected = input;

        let actual = remove_spaces_from(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_not_remove_newline_characters_but_should_remove_spaces() {
        let input = "a+ \n c";
        let expected = "a+\nc";

        let actual = remove_spaces_from(input);

        assert_eq!(actual, expected);
    }
}

macro_rules! all_ok {
    ($($i:ident: $t:ty = $e:expr),+) => {
        $(
            let $i: $t = match $e {
                Ok(x) => x,
                Err(e) => panic!("{}", e),
            };
        )+
    };
}

mod expression_parsing {
    use nom::{
        IResult, Parser, branch::alt, bytes::complete::tag, character::complete::digit1,
        combinator::opt, sequence::delimited,
    };

    use super::{BooleanExpression, BooleanOperation};
    use std::str::FromStr;

    pub fn boolean_expression(s: &str) -> IResult<&str, BooleanExpression> {
        let (_, (a, gate, b)) =
            delimited(opening_delim, (digit1, logic_gate, digit1), closing_delim).parse(s)?;

        // this macro is huge overkill but its cool so
        all_ok!(
            a_i8: i8 = i8::from_str(a),
            b_i8: i8 = i8::from_str(b)
        );

        Ok((
            s,
            BooleanExpression {
                raw: s,
                members: vec![a_i8, b_i8],
                operation: BooleanOperation {
                    operation_symbol: gate,
                },
            },
        ))
    }

    fn logic_gate(s: &str) -> IResult<&str, Option<&str>> {
        opt(alt((and_symbol, or_symbol))).parse(s)
    }

    fn and_symbol(s: &str) -> IResult<&str, &str> {
        tag(".").parse(s)
    }

    fn or_symbol(s: &str) -> IResult<&str, &str> {
        tag("+").parse(s)
    }

    fn opening_delim(s: &str) -> IResult<&str, Option<&str>> {
        opt(alt((open_paren, open_square, open_curly, open_angled))).parse(s)
    }

    fn closing_delim(s: &str) -> IResult<&str, Option<&str>> {
        opt(alt((close_paren, close_square, close_curly, close_angled))).parse(s)
    }

    fn open_paren(s: &str) -> IResult<&str, &str> {
        tag("(")(s)
    }

    fn close_paren(s: &str) -> IResult<&str, &str> {
        tag(")")(s)
    }

    fn open_square(s: &str) -> IResult<&str, &str> {
        tag("[")(s)
    }

    fn close_square(s: &str) -> IResult<&str, &str> {
        tag("]")(s)
    }

    fn open_curly(s: &str) -> IResult<&str, &str> {
        tag("{")(s)
    }

    fn close_curly(s: &str) -> IResult<&str, &str> {
        tag("}")(s)
    }

    fn open_angled(s: &str) -> IResult<&str, &str> {
        tag("<")(s)
    }

    fn close_angled(s: &str) -> IResult<&str, &str> {
        tag(">")(s)
    }

    #[cfg(test)]
    mod expression_parsing_tests {
        use super::*;

        #[test]
        fn should_parse_boolean_expression() {
            let input = "(1+0)";
            let expected = BooleanExpression {
                raw: input,
                members: vec![1, 0],
                operation: BooleanOperation {
                    operation_symbol: Some("+"),
                },
            };

            let (_, actual) = boolean_expression(input).unwrap();

            assert_eq!(actual, expected);
        }
    }
}
