use core::panic;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{prelude::*, BufReader};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CellStatus {
    Empty,
    Elf,
}

fn print_map(map: &Vec<Vec<CellStatus>>) {
    for line in map {
        println!(
            "{}",
            line.iter()
                .map(|cs| if *cs == CellStatus::Empty { "." } else { "#" })
                .join("")
        );
    }
}

fn add(x: usize, y: isize) -> usize {
    usize::try_from(isize::try_from(x).unwrap() + y).unwrap()
}

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() > 0)
        .collect();

    let original_width = lines[0].len();
    let original_height = lines.len();
    const PADDING: usize = 1000;

    let mut map: Vec<Vec<CellStatus>> =
        vec![vec![CellStatus::Empty; original_width + 2 * PADDING]; original_height + 2 * PADDING];

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            map[i + PADDING][j + PADDING] = match c {
                '#' => CellStatus::Elf,
                '.' => CellStatus::Empty,
                _ => panic!["Unreachable"],
            }
        }
    }

    //print_map(&map);

    let adj: Vec<(isize, isize)> = vec![
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, -1),
        (1, 1),
        (1, -1),
        (1, 0),
        (-1, 0),
    ];

    let movements: Vec<(Vec<(isize, isize)>, (isize, isize))> = vec![
        (vec![(-1, -1), (-1, 0), (-1, 1)], (-1, 0)),
        (vec![(1, -1), (1, 0), (1, 1)], (1, 0)),
        (vec![(-1, -1), (0, -1), (1, -1)], (0, -1)),
        (vec![(-1, 1), (0, 1), (1, 1)], (0, 1)),
    ];

    let mut current_first_possible_direction: isize = 0;

    for it in 0..1000 {
        let mut hm: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut remains: HashSet<(usize, usize)> = HashSet::new();
        for i in 0..(map.len()) {
            for j in 0..(map[0].len()) {
                if map[i][j] == CellStatus::Elf {
                    if adj
                        .iter()
                        .map(|(dx, dy)| (add(i, *dx), add(j, *dy)))
                        .all(|(i, j)| map[i][j] == CellStatus::Empty)
                    {
                        remains.insert((i, j));
                        continue;
                    }
                    let possible_movements: Vec<(isize, isize)> = (0..4)
                        .map(|possible_direction| {
                            usize::try_from(current_first_possible_direction + possible_direction)
                                .unwrap()
                                .rem_euclid(4)
                        })
                        .map(|i| &movements[i])
                        .filter(|(incs_check, _inc)| {
                            // Check if it can move
                            incs_check.iter().all(|(inc_x, inc_y)| {
                                map[add(i, *inc_x)][add(j, *inc_y)] == CellStatus::Empty
                            })
                        })
                        .map(|(_incs_check, inc)| *inc)
                        .collect();

                    if possible_movements.len() != 0 {
                        let inc = possible_movements.iter().next().unwrap();
                        let new_point = (add(i, inc.0), add(j, inc.1));
                        hm.insert((i, j), new_point);
                    } else {
                        remains.insert((i, j));
                    }
                }
            }
        }

        current_first_possible_direction += 1;
        //println!("After {} iteration(s): {:?}", it + 1, &hm);
        let mut new_map: Vec<Vec<CellStatus>> =
            vec![
                vec![CellStatus::Empty; original_width + 2 * PADDING];
                original_height + 2 * PADDING
            ];

        if hm.len() == 0 {
            println!("No elf need to move! Ending.");
            break;
        }
        for (i, j) in remains.iter() {
            new_map[*i][*j] = CellStatus::Elf;
        }
        for (origin, destination) in hm.iter() {
            if hm.values().filter(|v| *v == destination).count() <= 1 {
                new_map[destination.0][destination.1] = CellStatus::Elf;
            } else {
                new_map[origin.0][origin.1] = CellStatus::Elf;
            }
        }
        map = new_map.clone();
        println!("After {} iteration(s)", it + 1);
        //print_map(&map);
    }

    // up, left, down, right
    let mut limits = (usize::MAX,usize::MAX,usize::MIN,usize::MIN);

    for i in 0..(map.len()) {
        for j in 0..(map[0].len()) {
            if map[i][j] == CellStatus::Elf {
                if i < limits.0 {
                    limits.0 = i;
                }
                if i > limits.2 {
                    limits.2 = i
                }
                if j < limits.1 {
                    limits.1 = j
                }
                if j > limits.3 {
                    limits.3 = j
                }
            }  
        }
    }

    println!("Rounding box is: {:?}", limits);

    let mut result = 0;

    for i in limits.0..=limits.2 {
        for j in limits.1..=limits.3 {
            if map[i][j] == CellStatus::Empty{
                result += 1;
            }
        }
    }

    println!("Part 1 result: {}", result);
}
