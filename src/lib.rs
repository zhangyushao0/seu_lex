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
