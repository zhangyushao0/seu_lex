use super::ast;
enum Transition {
    Epsilon,
    Symbol(char),
}
#[derive(Clone)]
struct NfaState {
    transitions: Vec<(Transition, usize)>,
}

struct Nfa {
    states: Vec<NfaState>,
    start: usize,
    accept: usize,
    new_state: usize,
}

impl Nfa {
    fn new(pattern: String) -> Nfa {
        let ast = ast::Parser::new(pattern).parse();
        Nfa {
            states: Vec::new(),
            start: 0,
            accept: 0,
            new_state: 1,
        }
    }
    fn get_state(&mut self, state: usize) -> &mut NfaState {
        if state >= self.states.len() {
            self.states.resize(state + 1, NfaState { transitions: Vec::new() });
        }
        &mut self.states[state]
    }
    fn construct() {}
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
            ast::AstNode::O
        }
    }
}
