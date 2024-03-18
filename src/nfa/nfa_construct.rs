use super::ast;
enum Transition {
    Epsilon,
    Symbol(char),
}
struct NfaState {
    state_id: usize,
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
    fn construct() {}
    fn construct_node(&mut self, node: ast::AstNode) -> usize {
        match node {
            ast::AstNode::And(left, right) => {
                let left_state = self.construct_node(*left);
                let right_state = self.construct_node(*right);
                self.states[left_state]
                    .transitions
                    .push((Transition::Epsilon, right_state));
                left_state
            }
        }
    }
}
