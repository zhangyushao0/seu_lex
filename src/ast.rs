use display_tree::{AsTree, CharSet, DisplayTree, StyleBuilder};

#[derive(DisplayTree, Debug, Clone)]
pub enum AstNode {
    And(#[tree] Box<AstNode>, #[tree] Box<AstNode>),
    Or(#[tree] Box<AstNode>, #[tree] Box<AstNode>),
    Star(#[tree] Box<AstNode>),
    Plus(#[tree] Box<AstNode>),
    Question(#[tree] Box<AstNode>),
    Char(char),
    Span(char, char),
}

pub struct Parser {
    pattern: String,
}

#[derive(Eq, PartialEq, Debug)]
enum CharType {
    End,
    Begin,
    Middle,
    SpanMiddle,
    BackSlash,
    Normal,
}

impl Parser {
    pub fn new(pattern: String) -> Self {
        Parser { pattern }
    }
    fn add_dot(&mut self) {
        let check_char = |c: char| -> CharType {
            match c {
                '(' | '[' => CharType::Begin,
                ')' | ']' => CharType::End,
                '*' | '+' | '?' => CharType::End,
                '|' => CharType::Middle,
                '-' => CharType::SpanMiddle,
                _ => CharType::Normal,
            }
        };

        let mut new_pattern = String::new();
        let mut chars = self.pattern.chars();
        let mut prev = chars.next().unwrap();
        new_pattern.push(prev);
        for c in chars {
            let prev_type = check_char(prev);
            let cur_type = check_char(c);
            if prev_type == CharType::SpanMiddle || cur_type == CharType::SpanMiddle {
            } else if prev_type == CharType::Normal
                && (cur_type == CharType::Normal || cur_type == CharType::Begin)
            {
                new_pattern.push('.');
            } else if prev_type == CharType::End
                && (cur_type == CharType::Begin || cur_type == CharType::Normal)
            {
                new_pattern.push('.');
            }
            new_pattern.push(c);
            prev = c;
        }
        self.pattern = new_pattern;
    }
    fn to_postfix(&mut self) -> String {
        let mut postfix = String::new();
        let mut op_stack: Vec<char> = Vec::new();
        let precedence = |c: char| -> i32 {
            match c {
                '*' => 3,
                '.' => 2,
                '|' => 1,
                _ => 0,
            }
        };
        for c in self.pattern.chars() {
            match c {
                '(' => op_stack.push(c),
                ')' => {
                    while let Some(op) = op_stack.pop() {
                        if op == '(' {
                            break;
                        }
                        postfix.push(op);
                    }
                }
                '*' | '.' | '|' => {
                    while let Some(op) = op_stack.last() {
                        if precedence(c) <= precedence(*op) {
                            postfix.push(op_stack.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    op_stack.push(c);
                }
                _ => postfix.push(c),
            }
        }
        while let Some(op) = op_stack.pop() {
            postfix.push(op);
        }
        postfix
    }
    fn to_ast(&mut self) -> AstNode {
        let postfix = self.to_postfix();
        let mut stack: Vec<Box<AstNode>> = Vec::new();
        for c in postfix.chars() {
            match c {
                '.' => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Box::new(AstNode::And(left, right)));
                }
                '|' => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Box::new(AstNode::Or(left, right)));
                }
                '*' => {
                    let node = stack.pop().unwrap();
                    stack.push(Box::new(AstNode::Star(node)));
                }
                _ => {
                    stack.push(Box::new(AstNode::Char(c)));
                }
            }
        }
        *stack.pop().unwrap()
    }
    fn to_ast_directly(&mut self) -> AstNode {
        let mut stack: Vec<Box<AstNode>> = Vec::new();
        let mut op_stack: Vec<char> = Vec::new();
        let mut span_stack: Vec<char> = Vec::new();
        let precedence = |c: char| -> i32 {
            match c {
                '*' => 3,
                '+' => 3,
                '?' => 3,
                '.' => 2,
                '|' => 1,
                _ => 0,
            }
        };

        let stack_push = |op: char, stack: &mut Vec<Box<AstNode>>| match op {
            '.' => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(Box::new(AstNode::And(left, right)));
            }
            '|' => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(Box::new(AstNode::Or(left, right)));
            }
            '*' => {
                let node = stack.pop().unwrap();
                stack.push(Box::new(AstNode::Star(node)));
            }
            '+' => {
                let node = stack.pop().unwrap();
                stack.push(Box::new(AstNode::Plus(node)));
            }
            '?' => {
                let node = stack.pop().unwrap();
                stack.push(Box::new(AstNode::Question(node)));
            }
            _ => {}
        };

        for c in self.pattern.chars() {
            match c {
                '(' => op_stack.push(c),
                ')' => {
                    while let Some(op) = op_stack.pop() {
                        if op == '(' {
                            break;
                        }
                        stack_push(op, &mut stack);
                    }
                }
                '[' => {
                    span_stack.push(c);
                }
                '-' => {}
                ']' => {
                    while span_stack.last().unwrap() != &'[' {
                        let right = span_stack.pop().unwrap();
                        let left = span_stack.pop().unwrap();
                        stack.push(Box::new(AstNode::Span(left, right)));
                    }
                    span_stack.clear();
                }
                '*' | '+' | '?' | '.' | '|' => {
                    while let Some(op) = op_stack.last() {
                        if precedence(c) <= precedence(*op) {
                            stack_push(op_stack.pop().unwrap(), &mut stack);
                        } else {
                            break;
                        }
                    }
                    op_stack.push(c);
                }
                _ => {
                    if span_stack.is_empty() {
                        stack.push(Box::new(AstNode::Char(c)));
                    } else {
                        span_stack.push(c);
                    }
                }
            }
        }
        while let Some(op) = op_stack.pop() {
            stack_push(op, &mut stack);
        }
        *stack.pop().unwrap()
    }
    pub fn parse(&mut self) -> AstNode {
        self.add_dot();
        self.to_ast_directly()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_dot() {
        let mut parser = Parser::new("\"[a-f]|[A-F]|[0-9]*\"".to_string());
        parser.add_dot();
        println!("{}", parser.pattern);
    }

    #[test]
    fn test_to_postfix() {
        let mut parser = Parser::new("(a_b|a*b)*".to_string());
        parser.add_dot();
        let postfix = parser.to_postfix();
        println!("{}", postfix);
    }

    #[test]
    fn test_to_ast() {
        let mut parser = Parser::new("(a|b)*abb".to_string());
        parser.add_dot();
        let ast = parser.to_ast();
        println!(
            "{}",
            AsTree::new(&ast)
                .indentation(1)
                .char_set(CharSet::DOUBLE_LINE)
        );
    }

    #[test]
    fn test_to_ast_directly() {
        let mut parser = Parser::new("\"[!-~]*\"".to_string());
        parser.add_dot();
        let ast = parser.to_ast_directly();
        println!(
            "{}",
            AsTree::new(&ast)
                .indentation(1)
                .char_set(CharSet::DOUBLE_LINE)
        );
    }
}
