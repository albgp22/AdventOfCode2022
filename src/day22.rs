use core::panic;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

fn calculate_neighbors(
    m: usize,
    n: usize,
) -> HashMap<((isize, isize), Direction), ((isize, isize), Direction)> {
    let mut r: HashMap<((isize, isize), Direction), ((isize, isize), Direction)> = HashMap::new();
    for i in 0..m {
        for j in 0..n {
            let i = isize::try_from(i).unwrap();
            let j = isize::try_from(j).unwrap();
            match (i, j) {
                (199, 49) => {
                    r.insert(((i, j), Direction::Right), ((149, i - 100), Direction::Up));
                    r.insert(((i, j), Direction::Down), ((0, 100 + j), Direction::Down));
                }
                (149, 99) => {
                    r.insert(
                        ((i, j), Direction::Right),
                        ((149 - i, 149), Direction::Left),
                    );
                    r.insert(((i, j), Direction::Down), ((100 + j, 49), Direction::Left));
                }
                (49, 149) => {
                    r.insert(((i, j), Direction::Right), ((149 - i, 99), Direction::Left));
                    r.insert(((i, j), Direction::Down), ((j - 50, 99), Direction::Left));
                }
                (0, 149) => {
                    r.insert(((i, j), Direction::Up), ((199, j - 100), Direction::Up));
                    r.insert(((i, j), Direction::Right), ((149 - i, 99), Direction::Left));
                }
                (199, 0) => {
                    r.insert(((i, j), Direction::Down), ((0, 100 + j), Direction::Down));
                    r.insert(((i, j), Direction::Left), ((0, i - 100), Direction::Down));
                }
                (0, 50) => {
                    r.insert(((i, j), Direction::Up), ((100 + j, 0), Direction::Right));
                    r.insert(((i, j), Direction::Left), ((100 + i, 0), Direction::Right));
                }
                (100, 0) => {
                    r.insert(((i, j), Direction::Up), ((50 + j, 50), Direction::Right));
                    r.insert(((i, j), Direction::Left), ((i - 100, 50), Direction::Right));
                }
                (0..=49, 50) => {
                    r.insert(((i, j), Direction::Left), ((100 + i, 0), Direction::Right));
                }
                (50..=99, 50) => {
                    r.insert(((i, j), Direction::Left), ((100, i - 50), Direction::Down));
                }
                (100, 0..=49) => {
                    r.insert(((i, j), Direction::Up), ((50 + j, 50), Direction::Right));
                }
                (100..=149, 0) => {
                    r.insert(((i, j), Direction::Left), ((i - 100, 50), Direction::Right));
                }
                (150..=199, 0) => {
                    r.insert(((i, j), Direction::Left), ((0, i - 100), Direction::Down));
                }
                (199, 0..=49) => {
                    r.insert(((i, j), Direction::Down), ((0, 100 + j), Direction::Down));
                }
                (150..=199, 49) => {
                    r.insert(((i, j), Direction::Right), ((149, i - 100), Direction::Up));
                }
                (149, 50..=99) => {
                    r.insert(((i, j), Direction::Down), ((100 + j, 49), Direction::Left));
                }
                (100..=149, 99) => {
                    r.insert(
                        ((i, j), Direction::Right),
                        ((149 - i, 149), Direction::Left),
                    );
                }
                (50..=99, 99) => {
                    r.insert(((i, j), Direction::Right), ((49, i + 50), Direction::Up));
                }
                (49, 100..=149) => {
                    r.insert(((i, j), Direction::Down), ((j - 50, 99), Direction::Left));
                }
                (0..=49, 149) => {
                    r.insert(((i, j), Direction::Right), ((149 - i, 99), Direction::Left));
                }
                (0, 50..=99) => {
                    r.insert(((i, j), Direction::Up), ((100 + j, 0), Direction::Right));
                }
                (0, 100..=149) => {
                    r.insert(((i, j), Direction::Up), ((199, j - 100), Direction::Up));
                }
                (_, _) => {}
            }
        }
    }
    r
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

fn move_finger_2(
    map: &Vec<Vec<CellType>>,
    pos: (isize, isize),
    quantity: usize,
    d: Direction,
    cube_mapping: &HashMap<((isize, isize), Direction), ((isize, isize), Direction)>,
) -> ((isize, isize), Direction) {
    let mut pos = pos.clone();
    let mut d = d.clone();
    for _ in 0..quantity {
        //println!("Pos: {:?}", pos);
        let (new_pos, new_d) = if cube_mapping.contains_key(&((pos.0, pos.1), d)) {
            println!("Jumping from {:?}", (pos,d));
            let (new_pos, new_d) = *cube_mapping.get(&((pos.0, pos.1), d)).unwrap();
            println!("  to: {:?}", (new_pos,new_d));
            (new_pos, new_d)
        } else {
            (
                match d {
                    Direction::Down => (pos.0 + 1, pos.1),
                    Direction::Up => (pos.0 - 1, pos.1),
                    Direction::Right => (pos.0, pos.1 + 1),
                    Direction::Left => (pos.0, pos.1 - 1),
                },
                d,
            )
        };

        println!("{:?}", (new_pos, new_d));
        //println!("New pos: {:?}", new_pos);
        match map[usize::try_from(new_pos.0).unwrap()][usize::try_from(new_pos.1).unwrap()] {
            CellType::Empty => {
                panic!("Should be unreachable");
            }
            CellType::Walkable => {
                pos = new_pos;
                d = new_d;
            }
            CellType::Wall => break,
        }
    }
    println!("Returning: {:?}", (pos,d));
    (pos, d)
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
    let mut map = vec![vec![CellType::Empty; max_row_size]; lines.iter().len() - 1];

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

    let cube_mapping = calculate_neighbors(map.len(), map[0].len());

    assert_eq!(
        cube_mapping.get(&((50, 50), Direction::Left)).unwrap(),
        &((100, 0), Direction::Down)
    );
    assert_eq!(
        cube_mapping.get(&((99, 50), Direction::Left)).unwrap(),
        &((100, 49), Direction::Down)
    );

    assert_eq!(
        cube_mapping.get(&((100, 0), Direction::Up)).unwrap(),
        &((50, 50), Direction::Right)
    );
    assert_eq!(
        cube_mapping.get(&((100, 49), Direction::Up)).unwrap(),
        &((99, 50), Direction::Right)
    );

    assert_eq!(
        cube_mapping.get(&((150, 49), Direction::Right)).unwrap(),
        &((149, 50), Direction::Up)
    );
    assert_eq!(
        cube_mapping.get(&((199, 49), Direction::Right)).unwrap(),
        &((149, 99), Direction::Up)
    );

    assert_eq!(
        cube_mapping.get(&((149, 50), Direction::Down)).unwrap(),
        &((150, 49), Direction::Left)
    );
    assert_eq!(
        cube_mapping.get(&((149, 99), Direction::Down)).unwrap(),
        &((199, 49), Direction::Left)
    );

    assert_eq!(
        cube_mapping.get(&((99, 99), Direction::Right)).unwrap(),
        &((49, 149), Direction::Up)
    );
    assert_eq!(
        cube_mapping.get(&((50, 99), Direction::Right)).unwrap(),
        &((49, 100), Direction::Up)
    );

    assert_eq!(
        cube_mapping.get(&((49, 149), Direction::Down)).unwrap(),
        &((99, 99), Direction::Left)
    );
    assert_eq!(
        cube_mapping.get(&((49, 100), Direction::Down)).unwrap(),
        &((50, 99), Direction::Left)
    );

    assert_eq!(
        cube_mapping.get(&((100, 99), Direction::Right)).unwrap(),
        &((49, 149), Direction::Left)
    );
    assert_eq!(
        cube_mapping.get(&((149, 99), Direction::Right)).unwrap(),
        &((0, 149), Direction::Left)
    );

    assert_eq!(
        cube_mapping.get(&((0, 149), Direction::Right)).unwrap(),
        &((149, 99), Direction::Left)
    );
    assert_eq!(
        cube_mapping.get(&((49, 149), Direction::Right)).unwrap(),
        &((100, 99), Direction::Left)
    );

    assert_eq!(
        cube_mapping.get(&((199, 0), Direction::Down)).unwrap(),
        &((0, 100), Direction::Down)
    );
    assert_eq!(
        cube_mapping.get(&((199, 49), Direction::Down)).unwrap(),
        &((0, 149), Direction::Down)
    );

    assert_eq!(
        cube_mapping.get(&((0, 100), Direction::Up)).unwrap(),
        &((199, 0), Direction::Up)
    );
    assert_eq!(
        cube_mapping.get(&((0, 149), Direction::Up)).unwrap(),
        &((199, 49), Direction::Up)
    );

    assert_eq!(
        cube_mapping.get(&((150, 0), Direction::Left)).unwrap(),
        &((0, 50), Direction::Down)
    );
    assert_eq!(
        cube_mapping.get(&((199, 0), Direction::Left)).unwrap(),
        &((0, 99), Direction::Down)
    );

    assert_eq!(
        cube_mapping.get(&((0, 50), Direction::Up)).unwrap(),
        &((150, 0), Direction::Right)
    );
    assert_eq!(
        cube_mapping.get(&((0, 99), Direction::Up)).unwrap(),
        &((199, 0), Direction::Right)
    );

    assert_eq!(
        cube_mapping.get(&((0, 50), Direction::Left)).unwrap(),
        &((100, 0), Direction::Right)
    );
    assert_eq!(
        cube_mapping.get(&((49, 50), Direction::Left)).unwrap(),
        &((149, 0), Direction::Right)
    );

    assert_eq!(
        cube_mapping.get(&((100, 0), Direction::Left)).unwrap(),
        &((0, 50), Direction::Right)
    );
    assert_eq!(
        cube_mapping.get(&((149, 0), Direction::Left)).unwrap(),
        &((49, 50), Direction::Right)
    );

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

    let turnings_backup = turnings.clone();
    let mut turnings = turnings.iter();

    'main_loop: for m in movements_quantities.iter() {
        current_position = move_finger(
            &map,
            current_position,
            *m,
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
        1000 * (current_position.0 + 1)
            + 4 * (current_position.1 + 1)
            + match current_direction {
                Direction::Right => 0,
                Direction::Down => 1,
                Direction::Left => 2,
                Direction::Up => 3,
            }
    );

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

    let mut turnings = turnings_backup.iter();

    'main_loop2: for m in movements_quantities.iter() {
        //println!("{:?}", current_position);
        (current_position, current_direction) = move_finger_2(
            &map,
            current_position,
            *m,
            current_direction,
            &cube_mapping,
        );

        match turnings.next() {
            Some(t) => {
                current_direction = Direction::turn(current_direction, *t);
            }
            None => {
                break 'main_loop2;
            }
        }
    }
    println!("Final position: {:?}", current_position);
    println!("Final direction: {:?}", current_direction);
    println!(
        "Solution 2: {}",
        1000 * (current_position.0 + 1)
            + 4 * (current_position.1 + 1)
            + match current_direction {
                Direction::Right => 0,
                Direction::Down => 1,
                Direction::Left => 2,
                Direction::Up => 3,
            }
    );
}
