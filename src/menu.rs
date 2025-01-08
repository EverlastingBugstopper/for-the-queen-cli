use inquire::{InquireError, MultiSelect, Select};
use std::fmt::Display;

#[derive(Debug)]
pub struct MultiSelectMenu<T: Display + Copy> {
    title: String,
    options: Vec<(T, Checkbox)>,
}

impl<T: Display + Copy + PartialEq> MultiSelectMenu<T> {
    pub fn new(title: &str, options: impl IntoIterator<Item = T>) -> Self {
        Self {
            title: title.to_string(),
            options: options
                .into_iter()
                .map(|element| (element, Checkbox::Unchecked))
                .collect(),
        }
    }

    pub fn interact(&mut self) -> Result<(), InquireError> {
        let menu_view = self.view();
        let answer = MultiSelect::new(&self.title, menu_view.options)
            .with_default(&menu_view.selected_indexes)
            .prompt();

        match answer {
            Ok(selected) => {
                self.stringly_select(selected);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_selection_strings(&self) -> Vec<String> {
        self.get_selections()
            .iter()
            .map(|selection| selection.to_string())
            .collect()
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

    pub fn select(&mut self, selected_options: Vec<T>) {
        self.options.iter_mut().for_each(|(option, checkbox)| {
            *checkbox = if selected_options.contains(&option) {
                Checkbox::Checked
            } else {
                Checkbox::Unchecked
            };
        });
    }

    fn stringly_select(&mut self, selected_options: Vec<String>) {
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
}

pub struct MenuView {
    pub selected_indexes: Vec<usize>,
    pub options: Vec<String>,
    pub is_empty: bool,
}

#[derive(Debug)]
enum Checkbox {
    Checked,
    Unchecked,
}

#[derive(Debug)]
pub struct SingleSelectMenu<T: Display + Copy> {
    title: String,
    options: Vec<T>,
}

impl<T: Display + Copy> SingleSelectMenu<T> {
    pub fn new(title: &str, options: impl IntoIterator<Item = T>) -> Self {
        Self {
            title: title.to_string(),
            options: options.into_iter().collect(),
        }
    }

    pub fn interact(&mut self) -> Result<T, InquireError> {
        let answer = Select::new(
            &self.title,
            self.options
                .iter()
                .map(|option| option.to_string())
                .collect(),
        )
        .without_filtering()
        .prompt();

        match answer {
            Ok(selected) => Ok(*self
                .options
                .iter()
                .find(|option| option.to_string() == selected)
                .unwrap()),
            Err(e) => Err(e),
        }
    }
}
