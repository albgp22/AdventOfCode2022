use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let line0 = &lines[0];
    let chars: Vec<char> = line0.chars().into_iter().collect();
    let mut res: usize = 0;
    for i in 0..(chars.len()-4){
        let mut last_chars = HashSet::new();
        for j in i..(i+4){
            last_chars.insert(chars[j]);
        }
        if last_chars.len()==4{
            res=i+4;
            break
        }
    }
    println!("{}", res);

    let mut res: usize = 0;
    for i in 0..(chars.len()-14){
        let mut last_chars = HashSet::new();
        for j in i..(i+14){
            last_chars.insert(chars[j]);
        }
        if last_chars.len()==14{
            res=i+14;
            break
        }
    }
    println!("{}", res);
}