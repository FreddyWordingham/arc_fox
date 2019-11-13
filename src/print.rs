//! Print functions.

use colored::Colorize;
use terminal_size::terminal_size;

/// Report a value and either its associated name, or a human readable string if supplied.
#[macro_export]
macro_rules! report {
    ($expression: expr) => {
        log::info!("{: <31}: {}", stringify!($expression), $expression);
    };
    ($desc: tt, $expression: expr) => {
        log::info!("{: <31}: {}", $desc, $expression);
    };
    ($desc: tt, $expression: expr, $units: tt) => {
        log::info!("{: <31}: {} [{}]", $desc, $expression, $units);
    };
}

/// Print a main title bar.
pub fn title(title: &str) {
    let title = title.to_uppercase();

    let term_width = (terminal_size()
        .expect("Unable to determine the terminal size.")
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

    println!("  {}", "█".repeat(right_bar));
}

/// Print a section bar.
pub fn section(title: &str) {
    let term_width = (terminal_size()
        .expect("Unable to determine the terminal size.")
        .0)
        .0 as usize;

    print!("\n====");

    unsafe {
        /// Index of the section.
        static mut SECTION: i32 = 0;
        match SECTION % 6 {
            0 => print!(" {}", title.bright_red().bold()),
            1 => print!(" {}", title.bright_yellow().bold()),
            2 => print!(" {}", title.bright_green().bold()),
            3 => print!(" {}", title.bright_cyan().bold()),
            4 => print!(" {}", title.bright_blue().bold()),
            5 => print!(" {}", title.bright_magenta().bold()),
            _ => unreachable!(),
        }
        SECTION += 1;
    }

    let mut cur_len = 5 + title.len();

    if cur_len >= term_width {
        println!("");
        return;
    }

    print!(" ");
    cur_len += 1;
    while cur_len < term_width {
        print!("=");
        cur_len += 1;
    }

    println!("");
}
