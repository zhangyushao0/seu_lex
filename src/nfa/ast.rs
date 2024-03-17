enum AstNode {
    And(Box<AstNode>, Box<AstNode>),
    Or(Box<AstNode>, Box<AstNode>),
    Star(Box<AstNode>),
    Char(char),
}

struct Parser<I>
where
    I: Iterator<Item = char>,
{
    input: I,
    last_char: Option<char>,
    stack: Vec<AstNode>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = char>,
{
    fn new(mut input: I) -> Self {
        let last_char = input.next();
        Parser {
            input,
            last_char,
            stack: Vec::new(),
        }
    }
    fn step(&mut self) {
        self.last_char = self.input.next();
    }
    fn parse(&mut self) {
        while let Some(c) = self.last_char {
            match c {
                '(' => {
                    self.stack.push(AstNode::Char(c));
                    self.step();
                    self.parse();
                }
                ')' => {
                    self.parse_group();
                }
            }
        }
    }
    fn parse_group(&mut self) {
        let mut group = Vec::new();
        while let Some(c) = self.stack.pop() {
            match c {
                AstNode::Char('(') => {
                    self.stack.push(AstNode::Or(
                        Box::new(AstNode::Char('a')),
                        Box::new(AstNode::Char('b')),
                    ));
                    break;
                }
                _ => group.push(c),
            }
        }
    }
}
