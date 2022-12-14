
use std::fs::File;
use std::io::BufReader;

use std::io::{BufRead};
use std::io::{stdin};
use std::path::Path;

mod day1 ;mod day2 ;mod day3 ;mod day4 ;mod day5;
mod day6 ;mod day7 ;mod day8 ;mod day9 ;mod day10;
mod day11;mod day12;mod day13;mod day14;mod day15;
mod day16;          mod day18;mod day19;mod day20;
mod day21;mod day22;mod day23;mod day24;mod day25;

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
        13 => day13::solve(reader),
        14 => day14::solve(reader),
        15 => day15::solve(reader),
        16 => day16::solve(reader),
        18 => day18::solve(reader),
        19 => day19::solve(reader),
        20 => day20::solve(reader),
        21 => day21::solve(reader),
        22 => day22::solve(reader),
        23 => day23::solve(reader),
        24 => day24::solve(reader),
        25 => day25::solve(reader),
        _ => panic!("Not implemented"),
    }
}
