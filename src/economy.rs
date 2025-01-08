use crossterm::style::{style, Color, Stylize};
use inquire::InquireError;

use crate::{
    all_building_materials, all_clothing, all_complex_food, all_consumable_items,
    all_crafting_resources, all_fuel, all_services, all_simple_food, all_species, clear_screen,
    pluralize, titleize, wood, BuildingMaterial, MultiSelectMenu, Need, Recipe, Resource, Service,
    SingleSelectMenu, Species,
};

use std::{collections::BTreeMap, fmt::Display};

#[derive(Debug)]
pub struct Economy {
    pub species: MultiSelectMenu<Species>,
    pub services: MultiSelectMenu<Service>,
    pub fuel: MultiSelectMenu<Resource>,
    pub crafting_resources: MultiSelectMenu<Resource>,
    pub building_materials: MultiSelectMenu<Resource>,
    pub consumable_items: MultiSelectMenu<Resource>,
    pub simple_food: MultiSelectMenu<Resource>,
    pub complex_food: MultiSelectMenu<Resource>,
    pub clothing: MultiSelectMenu<Resource>,
    pub switcher: SingleSelectMenu<MenuKind>,
}

impl Default for Economy {
    fn default() -> Self {
        Self::new()
    }
}

impl Economy {
    pub fn new() -> Self {
        let mut economy = Self {
            species: MultiSelectMenu::new("Select your species:", all_species()),
            services: MultiSelectMenu::new("Select services you can provide:", all_services()),
            fuel: MultiSelectMenu::new("Select the you can produce:", all_fuel()),
            crafting_resources: MultiSelectMenu::new(
                "Select the crafting resources you can produce:",
                all_crafting_resources(),
            ),
            building_materials: MultiSelectMenu::new(
                "Select the building materials you can produce:",
                all_building_materials(),
            ),
            consumable_items: MultiSelectMenu::new(
                "Select the consumable items you can produce:",
                all_consumable_items(),
            ),
            simple_food: MultiSelectMenu::new(
                "Select the simple food you can produce:",
                all_simple_food(),
            ),
            complex_food: MultiSelectMenu::new(
                "Select the complex food you can produce:",
                all_complex_food(),
            ),
            clothing: MultiSelectMenu::new("Select the clothing you can produce:", all_clothing()),
            switcher: SingleSelectMenu::new("What would you like to do?\n", all_menus()),
        };

        economy.fuel.select(vec![wood()]);

        economy
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
            MenuKind::EditSimpleFood => self.edit_simple_food(),
            MenuKind::EditBuildingMaterials => self.edit_building_materials(),
            MenuKind::EditFuel => self.edit_fuel(),
            MenuKind::EditCraftingResources => self.edit_crafting_resources(),
            MenuKind::EditComplexFood => self.edit_complex_food(),
            MenuKind::EditClothing => self.edit_clothing(),
            MenuKind::EditConsumableItems => self.edit_consumable_items(),
            MenuKind::EditServices => self.edit_services(),
            MenuKind::EditSpecies => self.edit_species(),
        }
    }

    fn edit_simple_food(&mut self) -> Result<(), InquireError> {
        self.simple_food.interact()
    }

    fn edit_building_materials(&mut self) -> Result<(), InquireError> {
        self.building_materials.interact()
    }

    fn edit_fuel(&mut self) -> Result<(), InquireError> {
        self.fuel.interact()
    }

    fn edit_crafting_resources(&mut self) -> Result<(), InquireError> {
        self.crafting_resources.interact()
    }

    fn edit_complex_food(&mut self) -> Result<(), InquireError> {
        self.complex_food.interact()
    }

    fn edit_clothing(&mut self) -> Result<(), InquireError> {
        self.clothing.interact()
    }

    fn edit_consumable_items(&mut self) -> Result<(), InquireError> {
        self.consumable_items.interact()
    }

    fn edit_services(&mut self) -> Result<(), InquireError> {
        self.services.interact()
    }

    fn edit_species(&mut self) -> Result<(), InquireError> {
        self.species.interact()
    }

    fn print_needs(&self) {
        let mut need_counter: BTreeMap<Need, usize> = BTreeMap::new();
        let selected_species = self.species.get_selections();
        let num_species = selected_species.len();
        need_counter.insert(
            Need::BuildingMaterial(BuildingMaterial::Planks),
            num_species,
        );
        need_counter.insert(
            Need::BuildingMaterial(BuildingMaterial::Fabric),
            num_species,
        );
        need_counter.insert(
            Need::BuildingMaterial(BuildingMaterial::Bricks),
            num_species,
        );
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

        let mut need_count: Vec<(&Need, &usize)> = need_counter.iter().collect();
        need_count.sort_by(|a, b| b.1.cmp(a.1));

        let selected_facets = [
            self.services.get_selection_strings(),
            self.fuel.get_selection_strings(),
            self.crafting_resources.get_selection_strings(),
            self.building_materials.get_selection_strings(),
            self.consumable_items.get_selection_strings(),
            self.simple_food.get_selection_strings(),
            self.complex_food.get_selection_strings(),
            self.clothing.get_selection_strings(),
        ]
        .concat();

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
    EditFuel,
    EditCraftingResources,
    EditBuildingMaterials,
    EditConsumableItems,
    EditSimpleFood,
    EditComplexFood,
    EditClothing,
}

impl Display for MenuKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", titleize(self))
    }
}

fn all_menus() -> Vec<MenuKind> {
    vec![
        MenuKind::EditSimpleFood,
        MenuKind::EditBuildingMaterials,
        MenuKind::EditFuel,
        MenuKind::EditCraftingResources,
        MenuKind::EditComplexFood,
        MenuKind::EditClothing,
        MenuKind::EditConsumableItems,
        MenuKind::EditServices,
        MenuKind::EditSpecies,
    ]
}
