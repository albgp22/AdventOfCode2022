use std::fs::File;
use std::io::{prelude::*, BufReader};
use itertools::Itertools;

fn snafu2dec(s: String) -> i128 {
    let mut n = s.len();
    let mut ret = 0;
    for (i, c) in s.chars().rev().enumerate() {
        ret += (if c.to_string().parse::<i128>().is_ok() {
            c.to_string().parse::<i128>().unwrap()
        } else {
            match c {
                '-' => -1i128,
                '=' => -2i128,
                _ => !unreachable!(),
            }
        }) * 5i128.pow(i.try_into().unwrap());
    }
    ret
}

fn dec2snafu(i: i128) -> String {
    let mut chrs: Vec<String> = vec![];
    let mut i = i;
    while i != 0 {
        let digit = i % 5;
        i /= 5;
        match digit {
            0..=2 => chrs.push(format!["{}", digit]),
            3 => {
                chrs.push("=".to_string());
                i += 1;
            }
            4 => {
                chrs.push("-".to_string());
                i += 1;
            }
            _ => !unreachable!(),
        }
    }

    chrs.iter().rev().join("")
}

pub fn solve(reader: BufReader<File>) {
    assert!((1..1000).all(|i| i == snafu2dec(dec2snafu(i))));

    let result: i128 = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() > 0)
        .map(|l| snafu2dec(l))
        .sum();

    println!("{}", result);
    println!("{}", dec2snafu(result));
}
