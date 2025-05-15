mod logic_gates;
mod parsing;

#[cfg(test)]
mod integration_tests {
    use crate::parsing;

    #[test]
    fn should_evaluate_simple_or_expression_true() {
        let inputs = ["(1+0)", "(0+1)"];
        let expected_output = true;

        for input in inputs.iter() {
            let actual_output = parsing::evaluate_expression(input);

            assert_eq!(actual_output, expected_output);
        }
    }

    #[test]
    fn should_evaluate_simple_or_expression_false() {
        let input = "(0+0)";
        let expected_output = false;

        let actual_output = parsing::evaluate_expression(input);

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn should_evaluate_simple_and_expression_true() {
        let input = "(1.1)";
        let expected_output = true;

        let actual_output = parsing::evaluate_expression(input);

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn should_evaluate_simple_and_expression_false() {
        let inputs = ["(0.0)", "(1.0)", "(0.1)"];
        let expected_output = false;

        for input in inputs.iter() {
            let actual_output = parsing::evaluate_expression(input);

            assert_eq!(actual_output, expected_output);
        }
    }
}
