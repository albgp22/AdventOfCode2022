use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let re_stack = Regex::new(r"^(?:\s{4}|\[(.)\]\s)+(?:\s{3}|\[(.)\])$").unwrap();
    let re_numbers = Regex::new(r"^(\s\d\s\s)*(\s\d\s)$").unwrap();
    let re_action = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    let n_stacks = lines[0].chars().count() / 4 + 1;
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..n_stacks {
        stacks.insert(0, VecDeque::new())
    }

    for line in &lines {
        match 1 {
            _ if re_stack.is_match(line) => {
                let mut chars_it = line.chars().skip(1).step_by(4);
                for i in 0..n_stacks {
                    let c = chars_it.next().unwrap();
                    if c != ' ' {
                        stacks[i].push_front(c);
                    }
                }
            }
            _ if re_numbers.is_match(line) => {}
            _ if re_action.is_match(line) => {
                let matches = re_action.captures(line).unwrap();
                let quantity = matches.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let origin = matches.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let destination = matches.get(3).unwrap().as_str().parse::<usize>().unwrap();
                let mut tmp: VecDeque<char> = VecDeque::new();
                for _ in 0..quantity {
                    tmp.push_back(stacks[origin - 1].pop_back().unwrap());
                }
                stacks[destination - 1].extend(tmp);
            }
            _ => {}
        }
    }
    println!(
        "{}",
        stacks
            .iter()
            .map(|c| c.clone().pop_back().unwrap().to_string())
            .collect::<Vec<String>>()
            .join("")
    );

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..n_stacks {
        stacks.insert(0, VecDeque::new())
    }

    for line in &lines {
        match 1 {
            _ if re_stack.is_match(line) => {
                let mut chars_it = line.chars().skip(1).step_by(4);
                for i in 0..n_stacks {
                    let c = chars_it.next().unwrap();
                    if c != ' ' {
                        stacks[i].push_front(c);
                    }
                }
            }
            _ if re_numbers.is_match(line) => {}
            _ if re_action.is_match(line) => {
                let matches = re_action.captures(line).unwrap();
                let quantity = matches.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let origin = matches.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let destination = matches.get(3).unwrap().as_str().parse::<usize>().unwrap();
                let mut tmp: VecDeque<char> = VecDeque::new();
                for _ in 0..quantity {
                    tmp.push_front(stacks[origin - 1].pop_back().unwrap());
                }
                stacks[destination - 1].extend(tmp);
            }
            _ => {}
        }
    }
    println!(
        "{}",
        stacks
            .iter()
            .map(|c| c.clone().pop_back().unwrap().to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}
