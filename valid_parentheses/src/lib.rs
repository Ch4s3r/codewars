use std::collections::HashMap;

fn valid_parentheses(input: &str) -> bool {
    let opening_symbols = vec!['(', '[', '{', '<'];
    let closing_symbols = vec![')', ']', '}', '>'];

    let mut symbols = HashMap::new();
    symbols.insert('(', ')');
    symbols.insert('[', ']');
    symbols.insert('{', '}');
    symbols.insert('<', '>');

    let mut stack = vec![];
    for parentheses in input.chars() {
        if opening_symbols.contains(&parentheses) {
            stack.push(symbols.get(&parentheses).unwrap());
        }
        if closing_symbols.contains(&parentheses) && stack.pop() != Some(&parentheses) {
            return false;
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::valid_parentheses;

    #[test]
    fn sample_tests() {
        do_test("()", true);
        do_test("())", false);
        do_test("", true);
        do_test("(}{)", false);
        do_test("(){}{", false);
        do_test("((){})<>", true);
    }

    fn do_test(s: &str, exp: bool) {
        let actual = valid_parentheses(s);
        assert_eq!(
            actual, exp,
            "\nFor the input \"{}\", your result ({}) did not match the expected output ({})",
            s, actual, exp
        );
    }
}
