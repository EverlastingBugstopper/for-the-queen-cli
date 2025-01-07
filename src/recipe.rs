use crate::Good;

pub trait Recipe {
    fn recipe(&self) -> Vec<Vec<Good>>;
}
