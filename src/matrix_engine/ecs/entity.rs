use std::fmt::Display;

#[derive(Debug,Eq, Hash, PartialEq,Clone, Copy)]
pub struct Entity(pub(crate) usize);

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity[{}]",self.0)
    }
}