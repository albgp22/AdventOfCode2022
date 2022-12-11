use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut cycles = [1; 1000];
    let mut curr_cycle = 1;
    let mut res = 0;

    for line in &lines {
        let mut line_split = line.split(" ");
        match line_split.next().unwrap() {
            "noop" => {
                cycles[curr_cycle + 1] = cycles[curr_cycle];
            }
            "addx" => {
                cycles[curr_cycle + 1] = cycles[curr_cycle];
                cycles[curr_cycle + 2] =
                    cycles[curr_cycle] + line_split.next().unwrap().parse::<i32>().unwrap();
                curr_cycle  += 1;
                if curr_cycle % 40 == 20 {
                    res += (curr_cycle as i32)*cycles[curr_cycle];
                    println!("{}", cycles[curr_cycle]);
                }
            }
            _ => panic!(),
        }
        curr_cycle += 1;
        if curr_cycle % 40 == 20 {
            res += (curr_cycle as i32)*cycles[curr_cycle];
            println!("{}", cycles[curr_cycle]);
        }
    }
    println!("Result 1: {}", res);

    let mut cycles = [1; 1000];
    let mut curr_cycle = 1;

    for line in &lines {
        let px = (curr_cycle-1) as i32 % 40;
        if px >= cycles[curr_cycle]-1 && px <= cycles[curr_cycle]+1{
            print!("#");
        } else{
            print!(".");
        };
        if px % 39 == 0 && px != 0{
            println!("");
            
        };
       
        let mut line_split = line.split(" ");
        match line_split.next().unwrap() {
            "noop" => {
                cycles[curr_cycle + 1] = cycles[curr_cycle];
            }
            "addx" => {
                cycles[curr_cycle + 1] = cycles[curr_cycle];
                cycles[curr_cycle + 2] =
                    cycles[curr_cycle] + line_split.next().unwrap().parse::<i32>().unwrap();
                curr_cycle  += 1;
                let px = (curr_cycle-1) as i32 % 40;
                if px >= cycles[curr_cycle]-1 && px <= cycles[curr_cycle]+1 {
                    print!("#");
                } else{
                    print!(".");
                };
                if px % 40 == 39 {
                    println!("");
                };
            }
            _ => panic!(),
        }
        curr_cycle += 1;
    }
}
