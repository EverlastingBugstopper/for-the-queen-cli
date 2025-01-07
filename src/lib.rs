pub mod goods;
mod needs;
mod planned_economy;
mod recipe;
mod species;

use goods::*;
pub use needs::*;
pub use planned_economy::*;
pub use recipe::*;
pub use species::*;

use convert_case::{Case, Casing};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use std::fmt;
use std::io::{stdout, Write};

pub fn clear_screen() {
    let mut out = stdout();
    out.queue(Hide).unwrap();
    out.queue(Clear(ClearType::All)).unwrap();
    out.queue(MoveTo(0, 0)).unwrap();
    out.flush().unwrap();
}

pub fn restore_cursor() {
    let mut out = stdout();
    out.queue(Show).unwrap();
    out.flush().unwrap();
}

pub fn titleize(debuggable: impl fmt::Debug) -> String {
    format!("{:?}", debuggable).to_case(Case::Title)
}

pub fn pascalize(displayable: impl fmt::Display) -> String {
    format!("{}", displayable).to_case(Case::UpperCamel)
}

pub fn pluralize(options: &[impl fmt::Display], separator_word: impl fmt::Display) -> String {
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
