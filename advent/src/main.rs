use std::fs::read_to_string;

mod advent_day_five;
mod advent_day_four;
mod advent_day_one;
mod advent_day_three;
mod advent_day_two;

fn main() {
    println!("Hello");
}

pub fn read_input(path: &'static str) -> String {
    let input = read_to_string(path).unwrap();
    input
}
