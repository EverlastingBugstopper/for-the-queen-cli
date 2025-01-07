use crossterm::{
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use for_the_queen_cli::Economy;
use inquire::InquireError;

use std::io::{stdout, Write};

fn main() {
    clear_screen();
    let mut economy = Economy::default();
    let result = economy.plan();
    exit(result)
}

fn exit(result: Result<(), InquireError>) -> ! {
    clear_screen();
    restore_cursor();

    if let Err(e) = result {
        let message = match e {
            InquireError::OperationInterrupted | InquireError::OperationCanceled => {
                "We Do It All For Our Impatient Queen.".to_string()
            }
            InquireError::IO(e) => {
                format!("IO Error: {e}")
            }
            InquireError::NotTTY => {
                "You can't pipe stuff, this is an interactive program.".to_string()
            }
            InquireError::InvalidConfiguration(e) => {
                format!("Invalid configuration: {e}")
            }
            InquireError::Custom(e) => {
                format!("{e}")
            }
        };

        println!("{message}");
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}

fn clear_screen() {
    let mut out = stdout();
    out.queue(Hide).unwrap();
    out.queue(Clear(ClearType::All)).unwrap();
    out.queue(MoveTo(0, 0)).unwrap();
    out.flush().unwrap();
}

fn restore_cursor() {
    let mut out = stdout();
    out.queue(Show).unwrap();
    out.flush().unwrap();
}
