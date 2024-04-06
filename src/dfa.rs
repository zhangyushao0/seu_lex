enum Transition {
    Char(char),
}
struct DfaState {
    transitions: Vec<(Transition, usize)>,
}
struct Dfa {
    states: Vec<DfaState>,
}

impl Dfa {
    fn new() -> Self {
        Dfa { states: Vec::new() }
    }
}
