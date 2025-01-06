use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{all_species, pascalize, titleize, Need};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Species {
    Humans,
    Beavers,
    Lizards,
    Harpies,
    // Foxes,
    // Frogs
}

impl Species {
    pub fn needs(&self) -> Vec<Need> {
        match self {
            Self::Beavers => {
                vec![
                    Need::Biscuits,
                    Need::PickledGoods,
                    Need::Scrolls,
                    Need::Wine,
                    Need::Coats,
                ]
            }
            Self::Harpies => {
                vec![
                    Need::Jerky,
                    Need::Paste,
                    Need::Scrolls,
                    Need::Tea,
                    Need::Coats,
                    Need::Boots,
                ]
            }
            Self::Humans => {
                vec![
                    Need::Porridge,
                    Need::Biscuits,
                    Need::Pie,
                    Need::Incense,
                    Need::Ale,
                    Need::Coats,
                ]
            }
            Self::Lizards => {
                vec![
                    Need::Pie,
                    Need::PickledGoods,
                    Need::Jerky,
                    Need::Skewers,
                    Need::TrainingGear,
                    Need::Boots,
                ]
            } // Self::Foxes => {
              //     vec![

              //     ]
              // },
              // Self::Frogs => {
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
