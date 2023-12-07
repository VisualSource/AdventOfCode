use crate::Day;
use std::{env, fs};

pub mod aoc_cli;
pub mod commands;
pub mod readme_benchmarks;
pub mod runner;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

/// Helper function that reads a text file to a string.
///
/// # Panics
///
/// Will panic if the user has insufficient permissions to access the current directory.
#[must_use]
pub fn read_file(folder: &str, day: Day) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("data").join(folder).join(format!("{day}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

/// Creates the constant `DAY` and sets up the input and runner for each part.
#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        /// The current day.
        const DAY: advent::Day = advent::day!($day);

        fn main() {
            use advent::template::runner::*;
            let input = advent::template::read_file("inputs", DAY);
            run_part(part_one, &input, DAY, 1);
            run_part(part_two, &input, DAY, 2);
        }
    };
}