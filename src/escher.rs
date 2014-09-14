#![feature(globs)]
mod parser;
mod sexp;
fn main() {
    let a = parser::parse("\"ef\"");
    match a {
        Ok(a) => { println!("{}", a.sexp); }
        Err(message) => {println!("fail {}", message); }
    }
}