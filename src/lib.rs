mod ast;
mod common;
mod dfa;
mod lexer;
mod nfa;

use std::ffi::CStr;
use std::os::raw::c_char;
#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

use annotate_snippets::{Level, Renderer, Snippet};

lazy_static! {
    static ref TOKENS: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
    static ref INDEX: Mutex<usize> = Mutex::new(0);
}

#[no_mangle]
pub extern "C" fn lexer_init(lex_path: *const c_char, src_path: *const c_char) {
    let lex_path = unsafe { CStr::from_ptr(lex_path) };
    let src_path = unsafe { CStr::from_ptr(src_path) };
    let src_content = std::fs::read_to_string(src_path.to_str().unwrap()).unwrap();
    let tags = lexer::read_from_lex_file(lex_path.to_str().unwrap());
    let mut lexer = lexer::Lexer::new(src_content.chars(), tags);
    while let Some((token, tag)) = lexer.get_next_token() {
        TOKENS.lock().unwrap().push((token, tag.0));
    }
    if !lexer.is_done() {
        let content = src_content.chars().collect::<String>();
        let lines = content.lines().collect::<Vec<_>>();

        let error_pos = lexer.pos;

        let mut pos = 0;
        let mut line_no = 0;
        let mut col_no = 0;
        let error_token = src_content.chars().nth(error_pos - 1).unwrap();
        let error_message = format!("unexpected token for '{}'", error_token);
        for (i, line) in lines.iter().enumerate() {
            if pos + line.len() + 1 > error_pos {
                line_no = i;
                col_no = error_pos - pos;
                break;
            }
            pos += line.len() + 1;
        }

        let message = Level::Error.title("unrecongnized token").snippet(
            Snippet::source(src_content.as_str())
                .line_start(1)
                .origin(src_path.to_str().unwrap())
                .fold(true)
                .annotation(
                    Level::Error
                        .span(error_pos - 1..error_pos - 1)
                        .label(error_message.as_str()),
                ),
        );

        let renderer = Renderer::styled();
        println!("{}", renderer.render(message));
        panic!("Lexer error");
    }
}

#[no_mangle]
pub extern "C" fn lexer_get_tokens_count() -> usize {
    TOKENS.lock().unwrap().len()
}

#[no_mangle]
pub extern "C" fn lexer_get_token_name(index: usize) -> *const c_char {
    let tokens = TOKENS.lock().unwrap();
    let token = tokens.get(index).unwrap();
    let token_name = token.1.as_str();
    let c_str = std::ffi::CString::new(token_name).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn lexer_get_token_value(index: usize) -> *const c_char {
    let tokens = TOKENS.lock().unwrap();
    let token = tokens.get(index).unwrap();
    let token_value = token.0.as_str();
    let c_str = std::ffi::CString::new(token_value).unwrap();
    c_str.into_raw()
}
