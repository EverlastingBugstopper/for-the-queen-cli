use crate::{
    all_goods, all_services, all_species, pluralize, Good, Need, Recipe, Service, Species,
};

use std::{collections::BTreeMap, fmt::Display};

#[derive(Debug)]
pub struct PlannedEconomy {
    pub species: Menu<Species>,
    pub services: Menu<Service>,
    pub goods: Menu<Good>,
}

impl Default for PlannedEconomy {
    fn default() -> Self {
        Self::new()
    }
}

impl PlannedEconomy {
    pub fn new() -> Self {
        Self {
            species: Menu::new(all_species()),
            services: Menu::new(all_services()),
            goods: Menu::new(all_goods()),
        }
    }

    pub fn print_needs(&self) {
        let mut need_counter: BTreeMap<Need, i8> = BTreeMap::new();
        let selected_species = self.species.get_selections();
        let num_species = selected_species.len();
        selected_species.iter().for_each(|species| {
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
            if last_count != Some(count) && *count >= 2 {
                println!("-----------------------");
                println!("Needed by {count}/{} species", num_species);
                println!("-----------------------");
                last_count = Some(count);
            }
            println!(" > {need}");

            for ingredient_slot in need.recipe() {
                println!("  > {}", pluralize(&ingredient_slot, "or"));
                if ingredient_slot.len() == 1 {
                    // this is mostly for flour.
                    for nested_slot in ingredient_slot[0].recipe() {
                        println!("    > {}", pluralize(&nested_slot, "or"));
                    }
                }
            }
        }
        println!("-----------------------");
    }
}

pub struct MenuView {
    pub selected_indexes: Vec<usize>,
    pub options: Vec<String>,
    pub is_empty: bool,
}

#[derive(Debug)]
pub struct Menu<T: Display + Copy> {
    options: Vec<(T, Checkbox)>,
}

impl<T: Display + Copy> Menu<T> {
    pub fn new(options: impl IntoIterator<Item = T>) -> Self {
        Self {
            options: options
                .into_iter()
                .map(|element| (element, Checkbox::Unchecked))
                .collect(),
        }
    }

    pub fn select(&mut self, selected_options: Vec<String>) {
        self.options.iter_mut().for_each(|(option, checkbox)| {
            *checkbox = if selected_options.contains(&option.to_string()) {
                Checkbox::Checked
            } else {
                Checkbox::Unchecked
            };
        });
    }

    pub fn view(&self) -> MenuView {
        let mut is_empty = true;
        let mut selected_indexes = Vec::new();
        let mut options = Vec::new();

        for (i, (option, checkbox)) in self.options.iter().enumerate() {
            if let Checkbox::Checked = *checkbox {
                is_empty = false;
                selected_indexes.push(i);
            }
            options.push(option.to_string());
        }

        MenuView {
            is_empty,
            options,
            selected_indexes,
        }
    }

    pub fn get_selections(&self) -> Vec<T> {
        self.options
            .iter()
            .filter_map(|(option, checkbox)| {
                if let Checkbox::Checked = checkbox {
                    Some(*option)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug)]
enum Checkbox {
    Checked,
    Unchecked,
}
