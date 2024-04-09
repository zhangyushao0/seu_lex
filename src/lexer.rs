use crate::common::Tag;
use crate::dfa::Dfa;
pub struct Lexer<I>
where
    I: Iterator<Item = char>,
{
    pub input: I,
    dfa: Dfa,
    last_char: Option<char>,
    pub pos: usize,
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
        }
    }
    fn get_next_char(&mut self) -> Option<char> {
        if let Some(c) = self.last_char {
            self.last_char = None;
            return Some(c);
        }
        self.pos += 1;
        self.input.next()
    }
    pub fn get_next_token(&mut self) -> Option<(String, Tag)> {
        let mut state = 0;
        let mut token = String::new();
        // if let Some(c) = self.last_char {
        //     if let Some(next_state) = self.dfa.get_next_state(state, c) {
        //         token.push(c);
        //         state = next_state;
        //     } else {
        //         return None;
        //     }
        // }
        loop {
            if let Some(c) = self.get_next_char() {
                if let Some(next_state) = self.dfa.get_next_state(state, c) {
                    token.push(c);
                    state = next_state;
                } else {
                    self.last_char = Some(c);
                    break;
                }
            } else {
                self.last_char = None;
                break;
            }
        }
        if let Some(accept_tag) = self.dfa.states[state].accept.clone() {
            Some((token, accept_tag))
        } else {
            None
        }
    }
    pub fn is_done(&self) -> bool {
        self.last_char.is_none()
    }
}
pub fn read_from_lex_file(path: &str) -> Vec<(String, Tag)> {
    let mut pattern = Vec::new();
    let content = std::fs::read_to_string(path).unwrap();
    let mut lines = content.lines();
    while let Some(line) = lines.next() {
        let mut iter = line.split("->");
        let tag_str = iter.next().unwrap();
        let pattern_str = iter.next().unwrap();
        let pattern_str = pattern_str.replace("\\t", "\t");
        let pattern_str = pattern_str.replace("\\n", "\n");
        pattern.push((pattern_str.to_string(), Tag(tag_str.to_string())));
    }
    pattern
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexer() {
        let pattern = vec![
            (
                "(1|2|3|4|5|6|7|8|9)(0|1|2|3|4|5|6|7|8|9)*".to_string(),
                Tag("DIGIT".to_string()),
            ),
            (" ".to_string(), Tag("SPACE".to_string())),
        ];
        let input = "123 45".chars();
        let mut l = Lexer::new(input, pattern);
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
        let pattern = read_from_lex_file("/home/zys/repo/seu_lex/demo.lex");
        let input = "123 45 aufu0  ".chars();
        let mut l = Lexer::new(input, pattern);
        let mut tokens = Vec::new();
        while let Some(token) = l.get_next_token() {
            tokens.push(token);
        }
        println!("{:?}", tokens);
        assert!(l.is_done());
    }
}
