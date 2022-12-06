use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;

fn solve_for_n(n: usize, line: &str) -> usize{
    let mut res: usize = 0;
    let chars: Vec<char> = line.chars().into_iter().collect();
    for i in 0..(chars.len()-n){
        let mut last_chars = HashSet::new();
        for j in i..(i+n){
            last_chars.insert(chars[j]);
        }
        if last_chars.len()==n{
            res=i+n;
            break
        }
    }
    return res;
}

pub fn solve(reader: BufReader<File>) {
    let line: String = reader.lines().next().unwrap().unwrap();
    println!("{}", solve_for_n(4, &line));
    println!("{}", solve_for_n(14, &line));
}