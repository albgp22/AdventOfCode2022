use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Rem;
use std::thread::current;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellType {
    Empty,
    Wall,
    Walkable,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turning {
    Right,
    Left,
}
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn turn(origin: Self, td: Turning) -> Self {
        match td {
            Turning::Right => match origin {
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
            },
            Turning::Left => match origin {
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
            },
        }
    }
}

fn move_finger(
    map: &Vec<Vec<CellType>>,
    pos: (isize, isize),
    quantity: usize,
    d: Direction,
    n: isize,
    m: isize,
) -> (isize, isize) {
    let mut pos = pos.clone();
    for _ in 0..quantity {
        //println!("Pos: {:?}", pos);
        let mut new_pos = match d {
            Direction::Down => ((pos.0 + 1).rem_euclid(m), pos.1),
            Direction::Up => ((pos.0 - 1).rem_euclid(m), pos.1),
            Direction::Right => (pos.0, (pos.1 + 1).rem_euclid(n)),
            Direction::Left => (pos.0, (pos.1 - 1).rem_euclid(n)),
        };
        //println!("New pos: {:?}", new_pos);
        match map[usize::try_from(new_pos.0).unwrap()][usize::try_from(new_pos.1).unwrap()] {
            CellType::Empty => {
                //println!("{:?} was empty", new_pos);
                while map[usize::try_from(new_pos.0).unwrap()][usize::try_from(new_pos.1).unwrap()]
                    == CellType::Empty
                {
                    //println!("{}", new_pos.1 - 1);
                    new_pos = match d {
                        Direction::Down => ((new_pos.0 + 1).rem_euclid(m), new_pos.1),
                        Direction::Up => ((new_pos.0 - 1).rem_euclid(m), new_pos.1),
                        Direction::Right => (new_pos.0, (new_pos.1 + 1).rem_euclid(n)),
                        Direction::Left => (new_pos.0, (new_pos.1 - 1).rem_euclid(n)),
                    };
                    //println!("{:?}", d);
                    //println!("New pos: {:?}", new_pos);
                }
                match map[usize::try_from(new_pos.0).unwrap()][usize::try_from(new_pos.1).unwrap()]
                {
                    CellType::Wall => {
                        break;
                    }
                    CellType::Walkable => {
                        pos = new_pos;
                    }
                    _ => {
                        panic!("Shouldn't be reached");
                    }
                }
            }
            CellType::Walkable => {
                pos = new_pos;
            }
            CellType::Wall => break,
        }
    }
    pos
}

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() > 0)
        .collect();

    let max_row_size = lines
        .iter()
        .enumerate()
        .filter(|(i, _line)| *i != lines.len() - 1)
        .map(|(_i, l)| l.len())
        .max()
        .unwrap();
    let mut map = vec![vec![CellType::Empty; max_row_size]; lines.iter().len()-1];

    for (i, line) in lines
        .iter()
        .enumerate()
        .filter(|(i, _line)| *i != lines.len() - 1)
    {
        for (j, c) in line.chars().enumerate() {
            map[i][j] = match c {
                ' ' => CellType::Empty,
                '.' => CellType::Walkable,
                '#' => CellType::Wall,
                _ => panic!("Character {} not supported in map input", c),
            }
        }
    }

    let movements_quantities: Vec<usize> = lines[lines.len() - 1]
        .split(['R', 'L'])
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    let turnings: Vec<Turning> = lines[lines.len() - 1]
        .chars()
        .filter(|&c| c == 'R' || c == 'L')
        .map(|c| {
            if c == 'L' {
                Turning::Left
            } else {
                Turning::Right
            }
        })
        .collect();

    println!("Dimensions: ({},{})", map.len(), map[0].len());

    let mut current_position: (isize, isize) = (
        0,
        map[0]
            .iter()
            .enumerate()
            .filter(|(_i, c)| **c == CellType::Walkable)
            .map(|(i, _c)| isize::try_from(i).unwrap())
            .next()
            .unwrap(),
    );
    let mut current_direction = Direction::Right;

    println!("Initial position: {:?}", &current_position);

    let mut turnings = turnings.iter();

    'main_loop: for m in movements_quantities {
        current_position = move_finger(
            &map,
            current_position,
            m,
            current_direction,
            isize::try_from(max_row_size).unwrap(),
            isize::try_from(map.len()).unwrap(),
        );

        match turnings.next() {
            Some(t) => {
                current_direction = Direction::turn(current_direction, *t);
            }
            None => {
                break 'main_loop;
            }
        }
    }
    println!("Final position: {:?}", current_position);
    println!("Final direction: {:?}", current_direction);
    println!(
        "Solution 1: {}",
        1000 * (current_position.0+1)
            + 4 * (current_position.1+1)
            + match current_direction {
                Direction::Right => 0,
                Direction::Down => 1,
                Direction::Left => 2,
                Direction::Up => 3,
            }
    )
}
