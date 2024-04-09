use crate::common::Tag;
use crate::nfa::{self, Nfa};
use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;
use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Transition {
    Char(char),
}
#[derive(Debug)]
pub struct DfaState {
    transitions: Vec<(Transition, usize)>,
    nfa_states: Vec<usize>,
    pub accept: Option<Tag>,
}
#[derive(Debug)]
pub struct Dfa {
    pub states: Vec<DfaState>,
    nfa: Nfa,
}

impl Dfa {
    pub fn new(pattern: Vec<(String, Tag)>) -> Dfa {
        let mut nfa = Nfa::new(pattern);
        nfa.construct();
        Dfa {
            states: Vec::new(),
            nfa,
        }
    }
    pub fn get_next_state(&self, state: usize, c: char) -> Option<usize> {
        for (transition, next) in &self.states[state].transitions {
            let Transition::Char(ch) = transition;
            if *ch == c {
                return Some(*next);
            }
        }
        None
    }

    fn get_epsilon_closure(&self, state: usize) -> Vec<usize> {
        let mut closure = Vec::new();
        let mut stack = Vec::new();
        closure.push(state);
        stack.push(state);
        while let Some(s) = stack.pop() {
            for (t, next) in self.nfa.states[s].transitions.iter() {
                if let nfa::Transition::Epsilon = t {
                    if !closure.contains(next) {
                        closure.push(*next);
                        stack.push(*next);
                    }
                }
            }
        }
        closure
    }
    fn compute_nfa_transition_range(&self, states: Vec<usize>) -> Vec<char> {
        let mut range = Vec::new();
        for s in states {
            for (t, next) in self.nfa.states[s].transitions.iter() {
                if let nfa::Transition::Symbol(c) = t {
                    if !range.contains(c) {
                        range.push(*c);
                    }
                }
            }
        }
        range
    }
    fn calculate_accept(nfa_accept: Vec<(usize, Tag)>, states: Vec<usize>) -> Option<Tag> {
        let mut accepts = Vec::new();
        for state in states {
            for (accept, tag) in nfa_accept.iter() {
                if state == *accept {
                    accepts.push(tag.clone());
                }
            }
        }
        if accepts.is_empty() {
            None
        // } else if accepts.len() != 1 {
        //     panic!("Ambiguous accept state")
        } else {
            Some(accepts[0].clone())
        }
    }
    pub fn construct(&mut self) {
        let mut stack = Vec::new();
        let mut start = self.get_epsilon_closure(self.nfa.start);
        self.states.push(DfaState {
            transitions: Vec::new(),
            nfa_states: start.clone(),
            accept: None,
        });

        stack.push(self.states.len() - 1);

        while let Some(s) = stack.pop() {
            let range = self.compute_nfa_transition_range(self.states[s].nfa_states.clone());
            for c in range {
                let mut next = Vec::new();
                for nfa_state in self.states[s].nfa_states.clone() {
                    for (t, next_state) in self.nfa.states[nfa_state].transitions.iter() {
                        if let nfa::Transition::Symbol(ch) = t {
                            if *ch == c {
                                next.append(&mut self.get_epsilon_closure(*next_state));
                            }
                        }
                    }
                }
                if !next.is_empty() {
                    next.sort();
                    next.dedup();
                    if let Some(pos) = self
                        .states
                        .iter()
                        .position(|state| state.nfa_states == next)
                    {
                        self.states[s].transitions.push((Transition::Char(c), pos));
                    } else {
                        self.states.push(DfaState {
                            transitions: Vec::new(),
                            nfa_states: next.clone(),
                            accept: None,
                        });
                        stack.push(self.states.len() - 1);
                        let pos = self.states.len() - 1;
                        self.states[s].transitions.push((Transition::Char(c), pos));
                    }
                }
            }
        }
        for state in &mut self.states {
            state.accept =
                Self::calculate_accept(self.nfa.accept.clone(), state.nfa_states.clone());
        }
    }

    pub fn minimize(&mut self) {
        let mut partition = Vec::new();
        let mut accepts = HashMap::new();
        let mut reject = Vec::new();
        for (i, state) in self.states.iter().enumerate() {
            if let Some(accept_tag) = &state.accept {
                let entry = accepts.entry(accept_tag).or_insert(Vec::new());
                entry.push(i);
            } else {
                reject.push(i);
            }
        }
        for (_, accept) in accepts {
            partition.push(accept);
        }
        partition.push(reject);
        loop {
            let mut new_partition = Vec::new();
            let mut changed = false;

            for part in &partition {
                let mut group_map: HashMap<Vec<(Transition, usize)>, Vec<usize>> = HashMap::new();
                for &state_index in part {
                    let transitions = self.states[state_index].transitions.clone();
                    let group_transitions = transitions
                        .iter()
                        .map(|(t, next)| {
                            (
                                t.clone(),
                                partition.iter().position(|p| p.contains(next)).unwrap(),
                            )
                        })
                        .collect::<Vec<_>>();
                    let mut found_superset = false;
                    let mut superset_key = None;
                    for (key, value) in &mut group_map {
                        if group_transitions.iter().all(|item| key.contains(item)) {
                            value.push(state_index);
                            found_superset = true;
                            break;
                        } else if key.iter().all(|item| group_transitions.contains(item)) {
                            superset_key = Some(key.clone());
                            break;
                        }
                    }

                    if let Some(key) = superset_key {
                        if let Some(value) = group_map.remove(&key) {
                            let mut new_value = value;
                            new_value.push(state_index);
                            group_map.insert(group_transitions, new_value);
                        }
                    } else if !found_superset {
                        group_map.insert(group_transitions, vec![state_index]);
                    }
                }

                if group_map.len() > 1 {
                    changed = true;
                    for (_, group) in group_map {
                        new_partition.push(group);
                    }
                } else {
                    new_partition.push(part.clone());
                }
            }

            if !changed {
                break;
            }
            partition = new_partition;
        }

        partition.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut new_states = Vec::new();

        for part in &partition {
            let mut group_map: HashMap<Vec<(Transition, usize)>, Vec<usize>> = HashMap::new();
            for &state_index in part {
                let transitions = self.states[state_index].transitions.clone();
                let group_transitions = transitions
                    .iter()
                    .map(|(t, next)| {
                        (
                            t.clone(),
                            partition.iter().position(|p| p.contains(next)).unwrap(),
                        )
                    })
                    .collect::<Vec<_>>();

                let mut found_superset = false;
                let mut superset_key = None;
                for (key, value) in &mut group_map {
                    if group_transitions.iter().all(|item| key.contains(item)) {
                        value.push(state_index);
                        found_superset = true;
                        break;
                    } else if key.iter().all(|item| group_transitions.contains(item)) {
                        superset_key = Some(key.clone());
                        break;
                    }
                }

                if let Some(key) = superset_key {
                    if let Some(value) = group_map.remove(&key) {
                        let mut new_value = value;
                        new_value.push(state_index);
                        group_map.insert(group_transitions, new_value);
                    }
                } else if !found_superset {
                    group_map.insert(group_transitions, vec![state_index]);
                }
            }

            assert_eq!(group_map.len(), 1);
            let (transitions, _) = group_map.into_iter().next().unwrap();

            let mut accept = None;
            for state_index in part {
                if let Some(tag) = self.states[*state_index].accept.clone() {
                    accept = Some(tag);
                    break;
                }
            }

            new_states.push(DfaState {
                transitions,
                nfa_states: self.states[part[0]].nfa_states.clone(),
                accept,
            });
        }

        self.states = new_states;
    }

    fn to_graphviz(&self) -> String {
        let mut graph = DiGraph::<String, String>::new();
        let mut state_map = HashMap::new();

        for (i, state) in self.states.iter().enumerate() {
            let mut label = format!("State {}", i);

            if state.nfa_states.contains(&self.nfa.start) {
                label += " (start)";
            }

            if let Some(tag) = &state.accept {
                label += &format!(" (accept: {})", tag.0);
            }

            let node_index = graph.add_node(label);
            state_map.insert(i, node_index);
        }

        for (i, state) in self.states.iter().enumerate() {
            for (transition, next) in &state.transitions {
                let edge_label = match transition {
                    Transition::Char(c) => c.to_string(),
                };
                graph.add_edge(state_map[&i], state_map[next], edge_label);
            }
        }

        let mut dot = Dot::new(&graph);
        format!("{:?}", dot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_epsilon_closure() {
        let pattern = vec![
            ("(a_b|a*b)*".to_string(), Tag("a".to_string())),
            ("(a|b)*abb".to_string(), Tag("b".to_string())),
        ];
        let dfa = Dfa::new(pattern);
        let mut closure = dfa.get_epsilon_closure(14);
        closure.sort();
        println!("{:?}", closure);
    }
    #[test]
    fn test_compute_nfa_transition_range() {
        let pattern = vec![
            ("(a_b|a*b)*".to_string(), Tag("a".to_string())),
            ("(a|b)*abb".to_string(), Tag("b".to_string())),
        ];
        let dfa = Dfa::new(pattern);
        let range = dfa.compute_nfa_transition_range(vec![0, 1, 2]);
        println!("{:?}", range);
    }
    #[test]
    fn test_construct() {
        let pattern = vec![
            ("(a_b|a*b)*".to_string(), Tag("a".to_string())),
            ("(a|b)*abb".to_string(), Tag("b".to_string())),
        ];
        let mut dfa = Dfa::new(pattern);
        dfa.construct();
        for (i, state) in dfa.states.iter().enumerate() {
            println!("State {}", i);
            for (transition, next) in &state.transitions {
                println!(
                    "  {} -> State {}",
                    match transition {
                        Transition::Char(c) => c.to_string(),
                    },
                    next
                );
            }
        }
    }
    #[test]
    fn test_dfa_graphviz() {
        let pattern = vec![
            (
                "(1|2|3|4|5|6|7|8|9)(0|1|2|3|4|5|6|7|8|9)*".to_string(),
                Tag("DIGIT".to_string()),
            ),
            ("a".to_string(), Tag("SPACE".to_string())),
        ];
        let mut dfa = Dfa::new(pattern);
        dfa.construct();
        let dot = dfa.to_graphviz();
        println!("{}", dot);
    }

    #[test]
    fn test_minimize() {
        let pattern = vec![
            (
                "(1|2|3|4|5|6|7|8|9)(0|1|2|3|4|5|6|7|8|9)*".to_string(),
                Tag("DIGIT".to_string()),
            ),
            (" ".to_string(), Tag("SPACE".to_string())),
        ];
        let mut dfa = Dfa::new(pattern);
        dfa.construct();
        dfa.minimize();
        let dot = dfa.to_graphviz();
        println!("{}", dot);
    }
}
