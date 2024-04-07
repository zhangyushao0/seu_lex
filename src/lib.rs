mod ast;
mod common;
mod dfa;
mod lexer;
mod nfa;

#[no_mangle]
pub extern "C" fn init() {
    println!("init");
}
