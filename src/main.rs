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

fn pioneer_new_land() -> PlannedEconomy {
    PlannedEconomy::new()
}

fn adjust(planned_economy: &mut PlannedEconomy) {
    clear_screen();

    let menu_view = planned_economy.menu_view();

    planned_economy.print_needs();

    let title = if menu_view.is_empty {
        "Select your starting species:"
    } else {
        "Plan your economy:"
    };

    let formatter: MultiOptionFormatter<'_, String> =
        &|selected_species| pluralize(selected_species, "and");

    let answer = MultiSelect::new(title, menu_view.options)
        .with_help_message(
            "↑↓ to move, space to select, ← to reset, enter to continue, esc to exit",
        )
        .with_formatter(formatter)
        .with_default(&menu_view.selected_indexes)
        .prompt();

    match answer {
        Ok(selected) => planned_economy.select(selected),
        Err(e) => exit(e),
    }

    adjust(planned_economy)
}
