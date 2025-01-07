use crossterm::style::{style, Color, Stylize};
use inquire::InquireError;

use crate::{
    all_goods, all_services, all_species, clear_screen, pluralize, titleize, Good, MultiSelectMenu,
    Need, Recipe, Service, SingleSelectMenu, Species,
};

use std::{collections::BTreeMap, fmt::Display};

#[derive(Debug)]
pub struct Economy {
    pub species: MultiSelectMenu<Species>,
    pub services: MultiSelectMenu<Service>,
    pub goods: MultiSelectMenu<Good>,
    pub switcher: SingleSelectMenu<MenuKind>,
}

impl Default for Economy {
    fn default() -> Self {
        Self::new()
    }
}

impl Economy {
    pub fn new() -> Self {
        Self {
            species: MultiSelectMenu::new("Select your species:", all_species()),
            services: MultiSelectMenu::new("Select services you can provide:", all_services()),
            goods: MultiSelectMenu::new("Select goods you can produce:", all_goods()),
            switcher: SingleSelectMenu::new("What would you like to do?\n", all_menus()),
        }
    }

    pub fn plan(&mut self) -> Result<(), InquireError> {
        clear_screen();
        self.print_needs();
        if self.species.view().is_empty {
            self.edit_species()?;
        } else {
            self.switch_menus()?;
        }
        self.plan()
    }

    fn switch_menus(&mut self) -> Result<(), InquireError> {
        let menu_kind = self.switcher.interact()?;
        match menu_kind {
            MenuKind::EditGoods => self.edit_goods(),
            MenuKind::EditServices => self.edit_services(),
            MenuKind::EditSpecies => self.edit_species(),
        }
    }

    fn edit_species(&mut self) -> Result<(), InquireError> {
        self.species.interact()
    }

    fn edit_services(&mut self) -> Result<(), InquireError> {
        self.services.interact()
    }

    fn edit_goods(&mut self) -> Result<(), InquireError> {
        self.goods.interact()
    }

    fn print_needs(&self) {
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

        let selected_services = self.services.get_selection_strings();
        let selected_goods = self.goods.get_selection_strings();
        let selected_facets = [selected_goods, selected_services].concat();

        let mut last_count = None;
        for (need, count) in need_count {
            if *count >= 2 {
                if last_count != Some(count) {
                    println!("-----------------------");
                    println!("Needed by {count}/{} species", num_species);
                    println!("-----------------------");
                    last_count = Some(count);
                }

                println!(" > {}", colorize(need, &selected_facets));

                for ingredient_slot in need.recipe() {
                    let technicolor_ingredient_slot: Vec<String> = ingredient_slot
                        .iter()
                        .map(|ingredient| colorize(ingredient, &selected_facets))
                        .collect();
                    println!("  > {}", pluralize(&technicolor_ingredient_slot, "or"));
                    if ingredient_slot.len() == 1 {
                        // this is mostly for flour.
                        for nested_slot in ingredient_slot[0].recipe() {
                            let technicolor_nested_slot: Vec<String> = nested_slot
                                .iter()
                                .map(|ingredient| colorize(ingredient, &selected_facets))
                                .collect();
                            println!("    > {}", pluralize(&technicolor_nested_slot, "or"));
                        }
                    }
                }
            }
        }
        println!("-----------------------");
    }
}

fn colorize<T: Display>(facet: T, selected_facets: &[String]) -> String {
    let color = if selected_facets.contains(&facet.to_string()) {
        Color::Green
    } else {
        Color::Red
    };
    style(facet).with(color).to_string()
}

#[derive(Debug, Clone, Copy)]
pub enum MenuKind {
    EditSpecies,
    EditServices,
    EditGoods,
}

impl Display for MenuKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", titleize(self))
    }
}

fn all_menus() -> Vec<MenuKind> {
    vec![
        MenuKind::EditGoods,
        MenuKind::EditServices,
        MenuKind::EditSpecies,
    ]
}
