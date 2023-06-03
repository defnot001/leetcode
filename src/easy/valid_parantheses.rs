pub fn valid_parentheses(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for bracket in s.chars() {
        match bracket {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            closing => {
                if Some(closing) != stack.pop() {
                    return false;
                }
            }
        };
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::valid_parentheses;

    #[test]
    fn valid_parentheses_works() {
        let test1 = "()[]{}".to_string();
        let test2 = "(]".to_string();
        let test3 = "([({})])".to_string();

        assert_eq!(valid_parentheses(test1), true);
        assert_eq!(valid_parentheses(test2), false);
        assert_eq!(valid_parentheses(test3), true);
    }
}
