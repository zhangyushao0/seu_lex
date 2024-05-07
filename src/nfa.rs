use crate::ast;
use crate::common::Tag;
use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub enum Transition {
    Epsilon,
    Symbol(char),
}
#[derive(Clone, Debug)]
pub struct NfaState {
    pub transitions: Vec<(Transition, usize)>,
}

#[derive(Debug)]
pub struct Nfa {
    pub states: Vec<NfaState>,
    pub start: usize,
    pub accept: Vec<(usize, Tag)>,
    new_state: usize,
    asts: Vec<(ast::AstNode, Tag)>,
}

impl Nfa {
    pub fn new(pattern: Vec<(String, Tag)>) -> Nfa {
        let mut asts = Vec::new();
        for (s, t) in pattern {
            asts.push((ast::Parser::new(s).parse(), t));
        }
        Nfa {
            states: Vec::new(),
            start: 0,
            accept: Vec::new(),
            new_state: 0,
            asts,
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
    fn construct_one(&mut self, ast: ast::AstNode, tag: Tag) -> usize {
        let (start, accept) = self.construct_node(ast);
        self.accept.push((accept, tag));

        start
    }
    pub fn construct(&mut self) {
        self.get_state(self.start);
        self.new_state += 1;
        for (ast, tag) in &self.asts.clone() {
            let start = self.construct_one(ast.clone(), tag.clone());
            self.get_state(self.start)
                .transitions
                .push((Transition::Epsilon, start));
        }
        if self.states.len() <= self.new_state {
            self.states.resize(
                self.new_state,
                NfaState {
                    transitions: Vec::new(),
                },
            );
        }
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
            ast::AstNode::Plus(node) => {
                let start = self.new_state;
                self.new_state += 1;
                let accept = self.new_state;
                self.new_state += 1;
                let (node_start, node_accept) = self.construct_node(*node);
                self.get_state(start)
                    .transitions
                    .push((Transition::Epsilon, node_start));
                self.get_state(node_accept)
                    .transitions
                    .push((Transition::Epsilon, node_start));
                self.get_state(node_accept)
                    .transitions
                    .push((Transition::Epsilon, accept));
                (start, accept)
            }
            ast::AstNode::Question(node) => {
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
            ast::AstNode::Span(start_char, end_char) => {
                let start = self.new_state;
                self.new_state += 1;
                let accept = self.new_state;
                self.new_state += 1;
                for c in start_char..=end_char {
                    self.get_state(start)
                        .transitions
                        .push((Transition::Symbol(c as char), accept));
                }
                (start, accept)
            }
        }
    }

    fn to_graphviz(&self) -> String {
        let mut graph = DiGraph::<String, String>::new();
        let mut state_map = HashMap::new();
        for (i, state) in self.states.iter().enumerate() {
            let mut label = format!("State {}", i);
            if i == self.start {
                label += " (start)";
            }
            for (accept, tag) in &self.accept {
                if i == *accept {
                    label += &format!(" (accept: {})", tag.0);
                }
            }
            let node_index = graph.add_node(label);
            state_map.insert(i, node_index);
        }
        println!("{:?}", state_map);
        for (i, state) in self.states.iter().enumerate() {
            for (transition, next) in &state.transitions {
                let edge_label = match transition {
                    Transition::Epsilon => "ε".to_string(),
                    Transition::Symbol(c) => c.to_string(),
                };
                graph.add_edge(state_map[&i], state_map[next], edge_label);
            }
        }

        let mut dot = Dot::new(&graph);
        // dot.set_graph_id("nfa");
        format!("{:?}", dot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nfa_graphviz() {
        let pattern = vec![
            ("(a|b)+[a-c]".to_string(), Tag("a".to_string())),
            ("(a|b)*abb".to_string(), Tag("b".to_string())),
        ];
        let mut nfa = Nfa::new(pattern);
        nfa.construct();
        let dot = nfa.to_graphviz();
        println!("{}", dot);
    }
}
