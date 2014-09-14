#![feature(globs)]
mod parser;
mod sexp;
fn main() {
    let a = parser::parse("\"ef\"");
    println!("{}", a)
}