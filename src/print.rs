//! Print functions.

use colored::Colorize;
use terminal_size::terminal_size;

/// Print a main title bar.
pub fn title(title: &str) {
    let term_width = (terminal_size()
        .expect("Unable to determine terminal width.")
        .0)
        .0 as usize;

    let left_bar;
    let right_bar;

    if term_width < ((title.len() * 2) + 11) {
        left_bar = 4;
        right_bar = 4;
    } else {
        left_bar = (term_width - (title.len() * 2) - 3) / 2;
        right_bar = term_width - (title.len() * 2) - 3 - left_bar;
    }

    print!("{} ", "█".repeat(left_bar));

    let mut pos = 0;
    for ch in title.chars() {
        match pos % 6 {
            0 => print!(" {}", format!("{}", ch).bright_red().bold()),
            1 => print!(" {}", format!("{}", ch).bright_yellow().bold()),
            2 => print!(" {}", format!("{}", ch).bright_green().bold()),
            3 => print!(" {}", format!("{}", ch).bright_cyan().bold()),
            4 => print!(" {}", format!("{}", ch).bright_blue().bold()),
            5 => print!(" {}", format!("{}", ch).bright_magenta().bold()),
            _ => unreachable!(),
        }

        pos += 1;
    }

    print!("  {}", "█".repeat(right_bar));
}
