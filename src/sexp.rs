use std::collections::HashMap;

#[deriving(Eq, Show)]
pub enum Cell {
    IntCell(uint),
    IdentCell(String),
    StringCell(String),
    ListCell(Vec<Cell>)
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        match (*self, *other) {
            (IntCell(a), IntCell(b)) => {
                a == b
            }
            (StringCell(a), StringCell(b)) => {
                a == b
            }
            (IdentCell(a), IdentCell(b)) => {
                a == b
            }
            (ListCell(a), ListCell(b)) => {
                a == b
            }
            _ => {
                false
            }
        }
    }
}

#[deriving(Show)]
pub struct TypeSignature {
    types: Vec<Type>,
    parameters: Vec<ParameterType>
}

#[deriving(Show)]
pub struct Type {
    label: String
}

#[deriving(Show)]
pub struct ParameterType {
    label: String
}

#[deriving(Show)]
pub struct Sexp {
    sexp: Cell,
    signatures: HashMap<String, TypeSignature>
}
