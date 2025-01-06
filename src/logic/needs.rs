use serde::{Deserialize, Serialize};

use std::fmt;

use crate::titleize;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Need {
    Porridge,
    Biscuits,
    Pie,
    PickledGoods,
    Jerky,
    Paste,
    Skewers,
    Scrolls,
    Incense,
    Tea,
    Wine,
    Ale,
    TrainingGear,
    Coats,
    Boots,
}

impl fmt::Display for Need {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", titleize(self))
    }
}
