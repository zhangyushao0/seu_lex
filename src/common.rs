#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag(pub String);

pub fn input2internal(input_char: char) -> char {
    match input_char {
        '.' => '\u{1000}',
        '*' => '\u{1001}',
        '+' => '\u{1002}',
        '?' => '\u{1003}',
        '(' => '\u{1004}',
        ')' => '\u{1005}',
        '[' => '\u{1006}',
        ']' => '\u{1007}',
        '|' => '\u{1008}',
        '-' => '\u{1009}',
        '\\' => '\u{100a}',
        _ => input_char,
    }
}

pub fn internal2input(internal_char: char) -> char {
    match internal_char {
        '\u{1000}' => '.',
        '\u{1001}' => '*',
        '\u{1002}' => '+',
        '\u{1003}' => '?',
        '\u{1004}' => '(',
        '\u{1005}' => ')',
        '\u{1006}' => '[',
        '\u{1007}' => ']',
        '\u{1008}' => '|',
        '\u{1009}' => '-',
        '\u{100a}' => '\\',
        _ => internal_char,
    }
}
