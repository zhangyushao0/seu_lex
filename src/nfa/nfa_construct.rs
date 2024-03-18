use super::ast;
#[derive(Clone, Debug)]
enum Transition {
    Epsilon,
    Symbol(char),
}
#[derive(Clone, Debug)]
struct NfaState {
    transitions: Vec<(Transition, usize)>,
}

#[derive(Debug)]
struct Nfa {
    states: Vec<NfaState>,
    start: usize,
    accept: usize,
    new_state: usize,
    ast: ast::AstNode,
}

impl Nfa {
    fn new(pattern: String) -> Nfa {
        let ast = ast::Parser::new(pattern).parse();
        Nfa {
            states: Vec::new(),
            start: 0,
            accept: 0,
            new_state: 1,
            ast,
        }
    }
    fn get_state(&mut self, state: usize) -> &mut NfaState {
        if state >= self.states.len() {
            self.states.resize(
                state + 1,
                NfaState {
                    transitions: Vec::new(),
                },
            );
        }
        &mut self.states[state]
    }
    fn construct(&mut self) {
        let (start, accept) = self.construct_node(self.ast.clone());
        self.start = start;
        self.accept = accept;
    }
    fn construct_node(&mut self, node: ast::AstNode) -> (usize, usize) {
        match node {
            ast::AstNode::And(left, right) => {
                let (left_start, left_accept) = self.construct_node(*left);
                let (right_start, right_accept) = self.construct_node(*right);
                self.get_state(left_accept)
                    .transitions
                    .push((Transition::Epsilon, right_start));
                (left_start, right_accept)
            }
            ast::AstNode::Or(left, right) => {
                let start = self.new_state;
                self.new_state += 1;
                let accept = self.new_state;
                self.new_state += 1;
                let (left_start, left_accept) = self.construct_node(*left);
                let (right_start, right_accept) = self.construct_node(*right);
                self.get_state(start)
                    .transitions
                    .push((Transition::Epsilon, left_start));
                self.get_state(start)
                    .transitions
                    .push((Transition::Epsilon, right_start));
                self.get_state(left_accept)
                    .transitions
                    .push((Transition::Epsilon, accept));
                self.get_state(right_accept)
                    .transitions
                    .push((Transition::Epsilon, accept));
                (start, accept)
            }
            ast::AstNode::Star(node) => {
                let start = self.new_state;
                self.new_state += 1;
                let accept = self.new_state;
                self.new_state += 1;
                let (node_start, node_accept) = self.construct_node(*node);
                self.get_state(start)
                    .transitions
                    .push((Transition::Epsilon, node_start));
                self.get_state(start)
                    .transitions
                    .push((Transition::Epsilon, accept));
                self.get_state(node_accept)
                    .transitions
                    .push((Transition::Epsilon, node_start));
                self.get_state(node_accept)
                    .transitions
                    .push((Transition::Epsilon, accept));
                (start, accept)
            }
            ast::AstNode::Char(c) => {
                let start = self.new_state;
                self.new_state += 1;
                let accept = self.new_state;
                self.new_state += 1;
                self.get_state(start)
                    .transitions
                    .push((Transition::Symbol(c), accept));
                (start, accept)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nfa_construct() {
        let mut nfa = Nfa::new("(a|b)*abb".to_string());
        nfa.construct();
        for (i, state) in nfa.states.iter().enumerate() {
            println!("state {}", i);
            for (transition, next) in &state.transitions {
                match transition {
                    Transition::Epsilon => println!("  -> {}", next),
                    Transition::Symbol(c) => println!("  -{}-> {}", c, next),
                }
            }
        }
    }
}
