use crossterm::{
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

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
