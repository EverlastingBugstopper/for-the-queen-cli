use crate::Resource;

pub trait Recipe {
    fn recipe(&self) -> Vec<Vec<Resource>>;
}
