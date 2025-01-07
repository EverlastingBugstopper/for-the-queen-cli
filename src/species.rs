use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{pascalize, titleize, Clothing, ComplexFood, Need, Service};

pub fn all_species() -> Vec<Species> {
    vec![
        Species::Beavers,
        Species::Humans,
        Species::Harpies,
        Species::Lizards,
        Species::Foxes, // Species::Frogs
    ]
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Species {
    Humans,
    Beavers,
    Lizards,
    Harpies,
    Foxes,
    // Frogs
}

impl Species {
    pub fn needs(&self) -> Vec<Need> {
        match self {
            Self::Beavers => {
                vec![
                    Need::ComplexFood(ComplexFood::Biscuits),
                    Need::ComplexFood(ComplexFood::PickledGoods),
                    Need::Service(Service::Education),
                    Need::Service(Service::Luxury),
                    Need::Clothing(Clothing::Coats),
                ]
            }
            Self::Harpies => {
                vec![
                    Need::ComplexFood(ComplexFood::Jerky),
                    Need::ComplexFood(ComplexFood::Paste),
                    Need::Service(Service::Education),
                    Need::Service(Service::Treatment),
                    Need::Clothing(Clothing::Coats),
                    Need::Clothing(Clothing::Boots),
                ]
            }
            Self::Humans => {
                vec![
                    Need::ComplexFood(ComplexFood::Porridge),
                    Need::ComplexFood(ComplexFood::Biscuits),
                    Need::ComplexFood(ComplexFood::Pie),
                    Need::Service(Service::Religion),
                    Need::Service(Service::Leisure),
                    Need::Clothing(Clothing::Coats),
                ]
            }
            Self::Lizards => {
                vec![
                    Need::ComplexFood(ComplexFood::Pie),
                    Need::ComplexFood(ComplexFood::PickledGoods),
                    Need::ComplexFood(ComplexFood::Jerky),
                    Need::ComplexFood(ComplexFood::Skewers),
                    Need::Service(Service::Brawling),
                    Need::Clothing(Clothing::Boots),
                ]
            }
            Self::Foxes => {
                vec![
                    Need::ComplexFood(ComplexFood::Porridge),
                    Need::ComplexFood(ComplexFood::Skewers),
                    Need::ComplexFood(ComplexFood::PickledGoods),
                    Need::Clothing(Clothing::Boots),
                    Need::Service(Service::Religion),
                    Need::Service(Service::Treatment),
                ]
            } // Self::Frogs => {
              //     vec![

              //     ]
              // }
        }
    }
}

impl TryFrom<&String> for Species {
    type Error = std::io::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let result = all_species()
            .into_iter()
            .find(|species| species.to_string() == pascalize(value));

        if let Some(species) = result {
            Ok(species)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("{value} is not a valid species"),
            ))
        }
    }
}

impl fmt::Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", titleize(self))
    }
}
