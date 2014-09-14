#![feature(globs)]
mod parser;
mod sexp;
fn main() {
    let a = parser::parse("(define e (+ 2 4))e");
    println!("{}", a)
}