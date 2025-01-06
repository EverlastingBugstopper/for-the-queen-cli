mod logic;
pub use logic::{Need, Species};

use convert_case::{Case, Casing};
use crossterm::{
    cursor::{Hide, MoveTo},
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

pub fn titleize(debuggable: impl fmt::Debug) -> String {
    format!("{:?}", debuggable).to_case(Case::Title)
}

pub fn pascalize(displayable: impl fmt::Display) -> String {
    format!("{}", displayable).to_case(Case::UpperCamel)
}

pub fn pluralize(options: &[impl fmt::Display]) -> String {
    match options.len() {
        0 => "None".to_string(),
        1 => options[0].to_string(),
        2 => format!("{} and {}", options[0], options[1]),
        _ => {
            let (last, rest) = options.split_last().unwrap();
            format!(
                "{}, and {}",
                rest.into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                last
            )
        }
    }
}

pub fn all_species() -> Vec<Species> {
    let mut species = vec![
        Species::Beavers,
        Species::Humans,
        Species::Harpies,
        Species::Lizards,
    ];
    species.sort();
    species
}
