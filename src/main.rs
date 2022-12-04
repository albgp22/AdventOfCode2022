use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Lines;
use std::io::{self, BufRead};
use std::io::{stdin, stdout, Write};
use std::path::Path;

mod day1;
mod day2;

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
        day.parse().expect(&format!("{} is not an integer!", day))
    };

    let filename = format!("inputs/{}.txt", day);
    let file_path = Path::new(&filename);
    let display = file_path.display();
    //println!("{}", display);

    let file: File = File::open(&file_path).expect(&format!("Could not open {}", display));

    let mut reader: BufReader<File> = BufReader::new(file);

    match day {
        1 => day1::solve(reader),
        2 => day2::solve(reader),
        _ => print!(""),
    }
}
