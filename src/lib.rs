mod economy;
pub mod goods;
mod menu;
mod needs;
mod recipe;
mod species;

pub use economy::*;
use goods::*;
pub use menu::*;
pub use needs::*;
pub use recipe::*;
pub use species::*;

use convert_case::{Case, Casing};

use std::fmt::{Debug, Display};

pub fn titleize(debuggable: impl Debug) -> String {
    format!("{:?}", debuggable).to_case(Case::Title)
}

pub fn pascalize(displayable: impl Display) -> String {
    format!("{}", displayable).to_case(Case::UpperCamel)
}

pub fn pluralize(options: &[impl Display], separator_word: impl Display) -> String {
    match options.len() {
        0 => "None".to_string(),
        1 => options[0].to_string(),
        2 => format!("{a} {separator_word} {b}", a = options[0], b = options[1]),
        _ => {
            let (last, rest) = options.split_last().unwrap();
            format!(
                "{}, {separator_word} {}",
                rest.iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                last
            )
        }
    }
}
