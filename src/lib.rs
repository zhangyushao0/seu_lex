mod ast;
mod common;
mod dfa;
mod lexer;
mod nfa;

use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn init(c_string: *const c_char) {
    // 安全地将C字符串转换为Rust字符串切片
    let c_str = unsafe {
        assert!(!c_string.is_null());
        CStr::from_ptr(c_string)
    };

    // 将CStr转换为Rust的字符串切片，如果包含有效的UTF-8数据
    match c_str.to_str() {
        Ok(str_slice) => println!("Received string: {}", str_slice),
        Err(e) => eprintln!("Failed to convert C string to Rust string: {:?}", e),
    }
}
