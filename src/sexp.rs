#[deriving(Show)]
pub enum Cell {
    IntCell(uint),
    IdentCell(String),
    ListCell(Vec<Cell>)
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
