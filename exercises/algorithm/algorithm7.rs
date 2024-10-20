#[derive(Debug)]
struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn push(&mut self, value: T) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

fn is_valid_parentheses(s: &str) -> bool {
    let mut stack = Stack::new();

    for ch in s.chars() {
        match ch {
            '(' | '{' | '[' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_parentheses() {
        assert!(is_valid_parentheses("()"));
        assert!(is_valid_parentheses("()[]{}"));
        assert!(is_valid_parentheses("{[()]}"));
    }

    #[test]
    fn test_invalid_parentheses() {
        assert!(!is_valid_parentheses("(]"));
        assert!(!is_valid_parentheses("([)]"));
        assert!(!is_valid_parentheses("{[}"));
    }
}
