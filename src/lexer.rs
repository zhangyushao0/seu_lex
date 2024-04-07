use crate::common::Tag;
use crate::dfa::Dfa;
struct lexer<I>
where
    I: Iterator<Item = char>,
{
    input: I,
    dfa: Dfa,
    last_char: Option<char>,
}

impl<I> lexer<I>
where
    I: Iterator<Item = char>,
{
    fn new(input: I, pattern: Vec<(String, Tag)>) -> Self {
        let mut dfa = Dfa::new(pattern);
        dfa.construct();
        dfa.minimize();
        lexer {
            input,
            dfa,
            last_char: None,
        }
    }

    fn get_next_token(&mut self) -> Option<(String, Tag)> {
        let mut state = 0;
        let mut token = String::new();
        if let Some(c) = self.last_char {
            if let Some(next_state) = self.dfa.get_next_state(state, c) {
                token.push(c);
                state = next_state;
            } else {
                return None;
            }
        }
        loop {
            if let Some(c) = self.input.next() {
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
        let mut l = lexer::new(input, pattern);
        let mut tokens = Vec::new();
        while let Some(token) = l.get_next_token() {
            tokens.push(token);
        }
        println!("{:?}", tokens);
    }
}
