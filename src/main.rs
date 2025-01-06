use std::collections::HashMap;

use for_the_queen_cli::{all_species, clear_screen, pluralize, Need, Species};
use inquire::{formatter::MultiOptionFormatter, InquireError, MultiSelect, Select};

fn main() {
    let species = select_first_species();
    print_needs(&species);
    prompt_for_more_info(&species)
}

fn exit(e: InquireError) -> ! {
    let message = match e {
        InquireError::OperationInterrupted | InquireError::OperationCanceled => {
            "We Do It All For Our Impatient Queen.".to_string()
        },
        InquireError::IO(e) => {
            format!("IO Error: {e}")
        },
        InquireError::NotTTY => {
            format!("You can't pipe stuff, this is an interactive program.")
        },
        InquireError::InvalidConfiguration(e) => {
            format!("Invalid configuration: {e}")
        },
        InquireError::Custom(e) => {
            format!("{e}")
        }
    };
    clear_screen();
    println!("{message}");
    std::process::exit(1)
}

fn prompt_for_more_info(species: &[Species]) {
    let add_species = "Add Species";
    let add_building = "Add Building";
    let options = vec![add_species, add_building];

    let ans = Select::new("Actions:", options).with_help_message("↑↓ to move, enter to select, esc to exit").prompt();

    match ans {
        Ok(choice) => {
            if choice == add_species {
                let species = select_species(species);
                print_needs(&species);
                prompt_for_more_info(&species)
            } else if choice == add_building {
                unimplemented!()
            }
        }
        Err(e) => exit(e)
    }
}

fn print_needs(species: &[Species]) {
    let mut need_counter: HashMap<Need, i8> = HashMap::new();

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
    }
    println!("-----------------------");
}

fn select_first_species() -> Vec<Species> {
    select_species(&[])
}

fn select_species(preselected: &[Species]) -> Vec<Species> {
    clear_screen();

    let all_species = all_species();
    let mut preselected_indexes = Vec::with_capacity(preselected.len());
    for p in preselected {
        if let Ok(index) = all_species.binary_search(p) {
            preselected_indexes.push(index)
        }
    }

    let options: Vec<String> = all_species
        .iter()
        .map(|species| species.to_string())
        .collect();

    let formatter: MultiOptionFormatter<'_, String> =
        &|options| format!("{}", pluralize(&options));

    let answer = MultiSelect::new("Species:", options)
        .with_help_message("↑↓ to move, space to select, ← to reset, enter to continue, esc to exit")
        .with_formatter(formatter)
        .with_default(&preselected_indexes)
        .prompt();

    match answer {
        Ok(selected_species) => selected_species
            .iter()
            .map(|species| Species::try_from(species).unwrap())
            .collect(),
        Err(e) => exit(e),
    }
}
