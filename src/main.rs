use for_the_queen_cli::{clear_screen, pluralize, restore_cursor, PlannedEconomy};
use inquire::{formatter::MultiOptionFormatter, InquireError, MultiSelect};

fn main() {
    let mut planned_economy = pioneer_new_land();
    adjust(&mut planned_economy);
}

fn exit(e: InquireError) -> ! {
    let message = match e {
        InquireError::OperationInterrupted | InquireError::OperationCanceled => {
            "We Do It All For Our Impatient Queen.".to_string()
        }
        InquireError::IO(e) => {
            format!("IO Error: {e}")
        }
        InquireError::NotTTY => "You can't pipe stuff, this is an interactive program.".to_string(),
        InquireError::InvalidConfiguration(e) => {
            format!("Invalid configuration: {e}")
        }
        InquireError::Custom(e) => {
            format!("{e}")
        }
    };
    clear_screen();
    restore_cursor();
    println!("{message}");
    std::process::exit(1)
}

// fn state_of_the_union(species: &[Species]) {
//     let mut need_counter: HashMap<Need, i8> = HashMap::new();

//     species.iter().for_each(|species| {
//         species
//             .needs()
//             .iter()
//             .for_each(|need| match need_counter.get(need) {
//                 Some(need_count) => {
//                     need_counter.insert(*need, need_count + 1);
//                 }
//                 None => {
//                     need_counter.insert(*need, 1);
//                 }
//             })
//     });

//     let mut need_count: Vec<(&Need, &i8)> = need_counter.iter().collect();
//     need_count.sort_by(|a, b| b.1.cmp(a.1));

//     let mut last_count = None;
//     for (need, count) in need_count {
//         if last_count != Some(count) {
//             println!("-----------------------");
//             println!("Needed by {count}/{} species", species.len());
//             println!("-----------------------");
//             last_count = Some(count);
//         }
//         println!(" > {need}");

//         for ingredient_slot in need.recipe() {
//             for ingredient in &ingredient_slot {
//                 println!("  > {ingredient}");
//                 for nested_ingredient_slot in ingredient.recipe() {
//                     println!("    > {}", pluralize(&nested_ingredient_slot, "or"));
//                 }
//             }
//         }
//     }
//     println!("-----------------------");
// }

fn pioneer_new_land() -> PlannedEconomy {
    PlannedEconomy::new()
}

fn adjust(planned_economy: &mut PlannedEconomy) {
    clear_screen();

    let species_intel = planned_economy.species_intel();
    let services_intel = planned_economy.services_intel();

    let title = if species_intel.is_empty {
        "Select your starting species:"
    } else {
        "Plan your economy:"
    };

    let options = if species_intel.is_empty {
        species_intel.options
    } else {
        vec![species_intel.options, services_intel.options]
            .into_iter()
            .flatten()
            .collect()
    };

    let formatter: MultiOptionFormatter<'_, String> =
        &|selected_species| pluralize(&selected_species, "and");

    let answer = MultiSelect::new(title, options)
        .with_help_message(
            "↑↓ to move, space to select, ← to reset, enter to continue, esc to exit",
        )
        .with_formatter(formatter)
        .with_default(&species_intel.selected_indexes)
        .prompt();

    match answer {
        Ok(selected_species) => planned_economy.select_species(selected_species),
        Err(e) => exit(e),
    }

    adjust(planned_economy)
}
