use crate::{all_services, all_species, pascalize, titleize, Service, Species};

use std::fmt;

#[derive(Debug)]
pub struct PlannedEconomy {
    facets: Vec<(Facet, bool)>,
}

#[derive(Debug)]
pub enum Facet {
    Species(Species),
    Service(Service),
    // Good(Good)
}

impl fmt::Display for Facet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Species(species) => species.to_string(),
                Self::Service(service) => service.to_string(),
            }
        )
    }
}

impl PlannedEconomy {
    pub fn new() -> Self {
        Self {
            facets: all_species()
                .into_iter()
                .map(|species| (Facet::Species(species), false))
                .collect(),
        }
    }

    pub fn analyze(&self) -> Intel {
        let mut is_empty = true;
        let mut selected_indexes = Vec::new();
        let mut options = Vec::new();
        for (i, (facet, enabled)) in self.facets.iter().enumerate() {
            if (*enabled) {
                is_empty = false;
                selected_indexes.push(i);
            }
            options.push(facet.to_string());
        }

        Intel {
            is_empty,
            options,
            selected_indexes,
        }
    }
}

pub struct Intel {
    pub selected_indexes: Vec<usize>,
    pub options: Vec<String>,
    pub is_empty: bool,
}
