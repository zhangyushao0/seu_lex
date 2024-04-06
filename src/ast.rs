use display_tree::{AsTree, CharSet, DisplayTree, StyleBuilder};

#[derive(DisplayTree, Debug, Clone)]
pub enum AstNode {
    And(#[tree] Box<AstNode>, #[tree] Box<AstNode>),
    Or(#[tree] Box<AstNode>, #[tree] Box<AstNode>),
    Star(#[tree] Box<AstNode>),
    Char(char),
}

pub struct Parser {
    pattern: String,
}

impl Parser {
    pub fn new(pattern: String) -> Self {
        Parser { pattern }
    }
    fn add_dot(&mut self) {
        let mut new_pattern = String::new();
        let mut last_char = '\0';
        for c in self.pattern.chars() {
            if last_char == '\0' {
            } else if last_char != '('
                && last_char != '|'
                && c != '*'
                && c != ')'
                && c != '|'
                && c != '('
            {
                new_pattern.push('.');
            } else if c == '(' && last_char != '|' && last_char != '(' {
                new_pattern.push('.');
            }
            new_pattern.push(c);
            last_char = c;
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
        let precedence = |c: char| -> i32 {
            match c {
                '*' => 3,
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
                '*' | '.' | '|' => {
                    while let Some(op) = op_stack.last() {
                        if precedence(c) <= precedence(*op) {
                            stack_push(op_stack.pop().unwrap(), &mut stack);
                        } else {
                            break;
                        }
                    }
                    op_stack.push(c);
                }
                _ => stack.push(Box::new(AstNode::Char(c))),
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
        let mut parser = Parser::new("(a_b|a*b)*".to_string());
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
        let mut parser = Parser::new("(a|b)*abb".to_string());
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
