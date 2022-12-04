use std::fs::File;
use std::io::Lines;
use std::io::{self, prelude::*, BufReader};

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut res = 0;

    lines.iter().for_each(|l| {
        let mut spt = l.split(",");
        let elf1 = spt.next().unwrap();
        let elf2 = spt.next().unwrap();

        let elf1_range: Vec<i32> = elf1.split("-").map(|n| n.parse::<i32>().unwrap()).collect();
        let elf2_range: Vec<i32> = elf2.split("-").map(|n| n.parse::<i32>().unwrap()).collect();

        if (elf1_range[0] <= elf2_range[0] && elf1_range[1] >= elf2_range[1])
            || (elf2_range[0] <= elf1_range[0] && elf2_range[1] >= elf1_range[1])
        {
            res += 1
        }
    });

    println!("Full overlaps: {}", res);

    let mut res = 0;

    lines.iter().for_each(|l| {
        let mut spt = l.split(",");
        let elf1 = spt.next().unwrap();
        let elf2 = spt.next().unwrap();

        let elf1_range: Vec<i32> = elf1.split("-").map(|n| n.parse::<i32>().unwrap()).collect();
        let elf2_range: Vec<i32> = elf2.split("-").map(|n| n.parse::<i32>().unwrap()).collect();

        if (elf2_range[0] <= elf1_range[0] && elf2_range[1] >= elf1_range[0])
            || (elf2_range[0] <= elf1_range[1] && elf2_range[1] >= elf1_range[1])
            || (elf1_range[0] <= elf2_range[0] && elf1_range[1] >= elf2_range[0])
            || (elf1_range[0] <= elf2_range[1] && elf1_range[1] >= elf2_range[1])
        {
            res += 1
        }
    });

    println!("Overlaps: {}", res);
}
