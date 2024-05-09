use crate::common::{input2internal, internal2input, Tag};
use crate::dfa::Dfa;
pub struct Lexer<I>
where
    I: Iterator<Item = char>,
{
    pub input: I,
    dfa: Dfa,
    last_char: Option<char>,
    pub pos: usize,
    is_done: bool,
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(input: I, pattern: Vec<(String, Tag)>) -> Self {
        let mut dfa = Dfa::new(pattern);
        dfa.construct();
        dfa.minimize();
        Lexer {
            input,
            dfa,
            last_char: None,
            pos: 0,
            is_done: false,
        }
    }
    fn get_next_char(&mut self) -> Option<char> {
        if let Some(c) = self.last_char {
            self.last_char = None;
            let c = input2internal(c);
            return Some(c);
        }
        self.pos += 1;
        self.input.next().map(|c| input2internal(c))
    }
    pub fn get_next_token(&mut self) -> Option<(String, Tag)> {
        let mut state = 0;
        let mut token = String::new();
        let mut last_accept = None;
        loop {
            if let Some(c) = self.get_next_char() {
                if let Some(next_state) = self.dfa.get_next_state(state, c) {
                    token.push(c);
                    if let Some(accept_tag) = self.dfa.states[next_state].accept.clone() {
                        last_accept = Some((token.clone(), accept_tag));
                    }
                    state = next_state;
                } else {
                    self.last_char = Some(c);
                    break;
                }
            } else {
                self.last_char = None;
                if last_accept.is_some() {
                    self.is_done = true;
                }
                break;
            }
        }
        last_accept.map(|(token, tag)| (token.chars().map(|c| internal2input(c)).collect(), tag))
    }
    pub fn is_done(&self) -> bool {
        self.is_done
    }
}
pub fn read_from_lex_file(path: &str) -> Vec<(String, Tag)> {
    let mut pattern = Vec::new();
    let content = std::fs::read_to_string(path).unwrap();
    let mut lines = content.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        let mut iter = line.split("->");
        let tag_str = iter.next().unwrap();
        let pattern_str = iter.next().unwrap();
        let pattern_str = pattern_str.replace("\\t", "\t");
        let pattern_str = pattern_str.replace("\\n", "\n");
        let pattern_str = pattern_str.replace("\\r", "\r");
        let pattern_str = pattern_str.replace("\\.", "\u{1000}");
        let pattern_str = pattern_str.replace("\\*", "\u{1001}");
        let pattern_str = pattern_str.replace("\\+", "\u{1002}");
        let pattern_str = pattern_str.replace("\\?", "\u{1003}");
        let pattern_str = pattern_str.replace("\\(", "\u{1004}");
        let pattern_str = pattern_str.replace("\\)", "\u{1005}");
        let pattern_str = pattern_str.replace("\\[", "\u{1006}");
        let pattern_str = pattern_str.replace("\\]", "\u{1007}");
        let pattern_str = pattern_str.replace("\\|", "\u{1008}");
        let pattern_str = pattern_str.replace("\\-", "\u{1009}");
        let pattern_str = pattern_str.replace("\\\\", "\u{100a}");
        pattern.push((pattern_str.to_string(), Tag(tag_str.to_string())));
    }
    pattern
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexer() {
        let pattern = vec![("\"[a-c]*\"".to_string(), Tag("DIGIT".to_string()))];
        let input = "\"1.6f\"".chars();
        let mut l = Lexer::new(input, pattern);
        let graph = l.dfa.to_graphviz();
        println!("{}", graph);
        let mut tokens = Vec::new();
        while let Some(token) = l.get_next_token() {
            tokens.push(token);
        }
        println!("{:?}", tokens);
    }

    #[test]
    fn test_read_from_lex_file() {
        let pattern = read_from_lex_file("/home/zys/repo/seu_lex/demo.lex");
        println!("{:?}", pattern);
    }

    #[test]
    fn test_lexer_from_file() {
        let pattern = read_from_lex_file("G:\\repo\\seu_lex\\demo.lex");
        let input = "abc".chars();
        let mut l = Lexer::new(input, pattern);
        let mut tokens = Vec::new();
        while let Some(token) = l.get_next_token() {
            tokens.push(token);
        }
        println!("{:?}", tokens);
        assert!(l.is_done());
    }
}
