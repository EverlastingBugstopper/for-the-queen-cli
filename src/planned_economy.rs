use crate::{all_species, Need, Species};

use std::{collections::HashMap, fmt};

#[derive(Default, Debug)]
pub struct PlannedEconomy {
    facets: Vec<(Facet, bool)>,
}

#[derive(Debug)]
pub enum Facet {
    Species(Species),
    // Service(Service),
    // Good(Good)
}

impl TryFrom<&String> for Facet {
    type Error = std::io::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match Species::try_from(value) {
            Ok(species) => Ok(Facet::Species(species)),
            Err(e) => Err(e),
        }
    }
}

impl fmt::Display for Facet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Species(species) => species.to_string(),
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

    pub fn menu_view(&self) -> MenuView {
        let mut is_empty = true;
        let mut selected_indexes = Vec::new();
        let mut options = Vec::new();
        for (i, (facet, enabled)) in self.facets.iter().enumerate() {
            if *enabled {
                is_empty = false;
                selected_indexes.push(i);
            }
            options.push(facet.to_string());
        }

        MenuView {
            is_empty,
            options,
            selected_indexes,
        }
    }

    pub fn select(&mut self, selected: Vec<String>) {
        self.facets = all_species()
            .into_iter()
            .map(|species| {
                (
                    Facet::Species(species),
                    selected.contains(&species.to_string()),
                )
            })
            .collect();
    }

    pub fn species(&self) -> Vec<Species> {
        self.facets
            .iter()
            .filter_map(|(facet, enabled)| {
                if *enabled {
                    if let Facet::Species(species) = facet {
                        Some(*species)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn print_needs(&self) {
        let mut need_counter: HashMap<Need, i8> = HashMap::new();
        let species = self.species();

        species.iter().for_each(|species| {
            species
                .needs()
                .iter()
                .for_each(|need| match need_counter.get(need) {
                    Some(need_count) => {
                        need_counter.insert(*need, need_count + 1);
                    }
                    None => {
                        need_counter.insert(*need, 1);
                    }
                })
        });

        let mut need_count: Vec<(&Need, &i8)> = need_counter.iter().collect();
        need_count.sort_by(|a, b| b.1.cmp(a.1));

        let mut last_count = None;
        for (need, count) in need_count {
            if last_count != Some(count) {
                println!("-----------------------");
                println!("Needed by {count}/{} species", species.len());
                println!("-----------------------");
                last_count = Some(count);
            }
            println!(" > {need}");

            // for ingredient_slot in need.recipe() {
            //     for ingredient in &ingredient_slot {
            //         println!("  > {ingredient}");
            //         for nested_ingredient_slot in ingredient.recipe() {
            //             println!("    > {}", pluralize(&nested_ingredient_slot, "or"));
            //         }
            //     }
            // }
        }
        println!("-----------------------");
    }
}

pub struct MenuView {
    pub selected_indexes: Vec<usize>,
    pub options: Vec<String>,
    pub is_empty: bool,
}
