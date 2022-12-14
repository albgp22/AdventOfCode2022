
use std::fs::File;
use std::io::BufReader;

use std::io::{BufRead};
use std::io::{stdin};
use std::path::Path;

mod day1 ;mod day2;mod day3;mod day4;mod day5;
mod day6 ;mod day7;mod day8;mod day9;mod day10;
mod day11;mod day12;

fn main() {
    println!("Day:");
    let day: i32 = {
        let mut day = String::new();
        stdin()
            .read_line(&mut day)
            .expect("Did not enter a correct string");
        if let Some('\n') = day.chars().next_back() {
            day.pop();
        }
        if let Some('\r') = day.chars().next_back() {
            day.pop();
        }
        day.parse().unwrap_or_else(|_| panic!("{} is not an integer!", day))
    };

    let filename = format!("inputs/{}.txt", day);
    let file_path = Path::new(&filename);
    let display = file_path.display();
    //println!("{}", display);

    let file: File = File::open(&file_path).unwrap_or_else(|_| panic!("Could not open {}", display));

    let reader: BufReader<File> = BufReader::new(file);

    match day {
        1 => day1::solve(reader),
        2 => day2::solve(reader),
        3 => day3::solve(reader),
        4 => day4::solve(reader),
        5 => day5::solve(reader),
        6 => day6::solve(reader),
        7 => day7::solve(reader),
        8 => day8::solve(reader),
        9 => day9::solve(reader),
        10 => day10::solve(reader),
        11 => day11::solve(reader),
        12 => day12::solve(reader),
        _ => panic!("Not implemented"),
    }
}
