use super::Valamdo;
use doz::Name;

#[derive(Debug, PartialEq, Clone)]
pub struct Func {
    pub name: Name,
    pub body: Box<Valamdo>,
}
