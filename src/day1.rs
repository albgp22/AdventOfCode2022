use std::fs::File;
use std::io::{BufReader, Read};

pub fn solve(mut reader: BufReader<File>) {
    // 1st part
    let file_content = {
        let mut file_content = String::new();
        reader
            .read_to_string(&mut file_content)
            .expect("Couldn't read string");
        file_content
    };
    let maximum: i32 = file_content
        .split("\n\n")
        .map(|elf| elf.split('\n').map(|n| { n.parse::<i32>().unwrap_or(0) }).sum::<i32>())
        .max().unwrap();
    println!("The maximum is: {}", maximum);

    // 2nd part
    let mut cals: Vec<i32> = file_content
        .split("\n\n")
        .map(|elf| elf.split('\n').map(|n| { n.parse::<i32>().unwrap_or(0) }).sum::<i32>())
        .collect::<Vec<i32>>();
    cals.sort();

    println!("The sum of three maximums is: {}", cals.iter().rev().take(3).sum::<i32>());
}
