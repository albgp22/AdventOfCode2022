use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::io::Lines;

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res: i32 = lines.iter().map(
        |ll| {
            let mut res = 0;
            match ll.chars().nth(2).unwrap(){
                'X' => {
                    res+=1;
                    match ll.chars().nth(0).unwrap(){
                        'A' => res+=3,
                        'C' => res+=6,
                        _ => {}
                    }
                },
                'Y' => {
                    res+=2;
                    match ll.chars().nth(0).unwrap(){
                        'B' => res+=3,
                        'A' => res+=6,
                        _ => {}
                    }
                },
                'Z' => {
                    res+=3;
                    match ll.chars().nth(0).unwrap(){
                        'C' => res+=3,
                        'B' => res+=6,
                        _ => {}
                    }
                },
                _ => {}
            }
            res
        }
    ).sum();
    println!("Total score is: {}", res);
    
    let res: i32 = lines.iter().map(
        |ll| {
            let mut res = 0;
            match ll.chars().nth(2).unwrap(){
                'Y' => {
                    res+=3;
                    res += match ll.chars().nth(0).unwrap(){
                            'A' => 1,
                            'B' => 2,
                            'C' => 3,
                            _ => panic!(),
                        };
                },
                'X' => {
                    res += match ll.chars().nth(0).unwrap(){
                        'A' => 3,
                        'B' => 1,
                        'C' => 2,
                        _ => panic!(),
                    };
                },
                'Z' => {
                    res+=6;
                    res += match ll.chars().nth(0).unwrap(){
                        'A' => 2,
                        'B' => 3,
                        'C' => 1,
                        _ => panic!(),
                    };
                },
                _ => panic!(),
            };
            res
            
        }
    ).sum();
    println!("Total score is: {}", res);    
}