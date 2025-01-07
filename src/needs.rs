use serde::{Deserialize, Serialize};

use std::fmt::{self, Display};

use crate::{goods::*, titleize, Clothing, ComplexFood, Good, Recipe};

#[derive(Ord, PartialOrd, Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Need {
    Clothing(Clothing),
    ComplexFood(ComplexFood),
    Service(Service),
}

impl Recipe for Need {
    fn recipe(&self) -> Vec<Vec<Good>> {
        match self {
            Self::Clothing(clothing) => clothing.recipe(),
            Self::ComplexFood(complex_food) => complex_food.recipe(),
            Self::Service(service) => service.recipe(),
        }
    }
}

impl Display for Need {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Clothing(clothing) => titleize(clothing),
                Self::ComplexFood(complex_food) => titleize(complex_food),
                Self::Service(service) => titleize(service),
            }
        )
    }
}

#[derive(Ord, PartialOrd, Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Service {
    Education,
    Religion,
    Treatment,
    Luxury,
    Leisure,
    Brawling,
}

impl Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", titleize(self))
    }
}

pub fn all_services() -> Vec<Service> {
    vec![
        Service::Education,
        Service::Religion,
        Service::Treatment,
        Service::Luxury,
        Service::Leisure,
        Service::Brawling,
    ]
}

pub fn education() -> Need {
    Need::Service(Service::Education)
}

pub fn religion() -> Need {
    Need::Service(Service::Religion)
}

pub fn treatment() -> Need {
    Need::Service(Service::Treatment)
}

pub fn luxury() -> Need {
    Need::Service(Service::Luxury)
}

pub fn leisure() -> Need {
    Need::Service(Service::Leisure)
}

pub fn brawling() -> Need {
    Need::Service(Service::Brawling)
}

impl Recipe for Service {
    fn recipe(&self) -> Vec<Vec<Good>> {
        vec![vec![match self {
            Self::Education => scrolls(),
            Self::Religion => incense(),
            Self::Treatment => tea(),
            Self::Luxury => wine(),
            Self::Leisure => ale(),
            Self::Brawling => training_gear(),
        }]]
    }
}
