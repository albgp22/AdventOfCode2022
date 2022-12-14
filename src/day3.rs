
use std::fs::File;

use std::io::{prelude::*, BufReader};

fn find_common_char(backpack: &str) -> char {
    let middle_point = backpack.chars().count() / 2;
    let containers: Vec<char> = backpack.chars().collect();
    let c1: Vec<char> = containers[..middle_point].to_vec();
    let c2: Vec<char> = containers[middle_point..].to_vec();

    *c1.iter().find(|x| c2.contains(x)).unwrap()
}

fn to_prio(c: char) -> u32 {
    let cu = c as u32;
    match cu{
        65..=90 => {
            cu - 65 + 27
        },
        97..=122 => {
            cu - 97 + 1
        },
        _ => panic!()
    }
}

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res: u32 = lines
        .iter()
        .map(|l| find_common_char(l))
        .map(to_prio )
        .sum();
    println!("Sum of priorities: {}", res);

    let mut res = 0;
    let mut found: bool;
    
    for (i, l) in lines.iter().enumerate(){
        match i%3 {
            0 => {
                found = false;
                l.chars().for_each(
                    |c| {
                        if !found && lines[i+1].contains(c) && lines[i+2].contains(c) {
                            res += to_prio(c);
                            found = !found;
                        }
                    }
                );
            },
            _ => {}
        }
    }
    println!("Sum of priorities for common groups: {}", res);
}
