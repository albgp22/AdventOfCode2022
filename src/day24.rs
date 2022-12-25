use core::panic;
use num::integer::lcm;
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn neighbors(
    p: &Vec<Vec<(i32, i32, Direction)>>,
    (i, j): (i32, i32),
    period: i32,
    epoch: i32,
    (height, width): (i32, i32),
) -> Vec<(i32, i32)> {
    let r = vec![(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1), (i, j)]
        .iter()
        .filter(|(i, j)| {
            !p[usize::try_from((epoch + 1).rem_euclid(period)).unwrap()]
                .iter()
                .any(|(ii, jj, _dd)| ii == i && jj == j)
        })
        .filter(|(i, j)| (*i >= 0 && *j >= 0) || ((*i, *j) == (-1, 0)))
        .filter(|(i, j)| (*i < height && *j < width) || ((*i, *j) == (height, width - 1)))
        .map(|(i, j)| (*i, *j))
        .collect::<Vec<_>>();
    r
}

fn distance(
    p: &Vec<Vec<(i32, i32, Direction)>>,
    src: (i32, i32),
    dest: (i32, i32),
    period: i32,
    (height, width): (i32, i32),
    starting_epoch: i32,
) -> i32 {
    let mut m: HashSet<(i32, i32)> = HashSet::new();
    m.insert(src);
    let mut c = starting_epoch;
    loop {
        let mut m2 = HashSet::new();
        for prev in m.iter() {
            for n in neighbors(p, *prev, period, c, (height, width)) {
                m2.insert(n);
            }
        }
        c += 1;
        m = m2;
        if m.contains(&dest) {
            return c;
        }
    }
}

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() > 0)
        .collect();

    let mut status_minute: Vec<Vec<(i32, i32, Direction)>> = vec![];
    let mut initial_points: Vec<(i32, i32, Direction)> = vec![];

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let (i, j): (i32, i32) = (i.try_into().unwrap(), j.try_into().unwrap());
            let (i, j) = (i - 1, j - 1);
            match c {
                '^' => initial_points.push((i, j, Direction::Up)),
                '>' => initial_points.push((i, j, Direction::Right)),
                '<' => initial_points.push((i, j, Direction::Left)),
                'v' => initial_points.push((i, j, Direction::Down)),
                '#' | '.' => {}
                _ => panic! {},
            }
        }
    }

    let width: i32 = lines[0].len().try_into().unwrap();
    let height: i32 = lines.len().try_into().unwrap();

    println!("Map size is: ({},{})", width, height);
    let (width, height) = (width - 2, height - 2);

    status_minute.push(initial_points);

    let period = lcm(width, height);

    for _minute in 1i32..period {
        let mut new_status = vec![];
        for (i, j, d) in &status_minute[status_minute.len() - 1] {
            new_status.push(match d {
                Direction::Up => {
                    let mut new_point = (*i - 1, *j, *d);
                    if new_point.0 == -1 {
                        new_point.0 = height - 1;
                    }
                    new_point
                }
                Direction::Down => {
                    let mut new_point = (*i + 1, *j, *d);
                    if new_point.0 == height {
                        new_point.0 = 0;
                    }
                    new_point
                }
                Direction::Left => {
                    let mut new_point = (*i, *j - 1, *d);
                    if new_point.1 == -1 {
                        new_point.1 = width - 1;
                    }
                    new_point
                }
                Direction::Right => {
                    let mut new_point = (*i, *j + 1, *d);
                    if new_point.1 == width {
                        new_point.1 = 0;
                    }
                    new_point
                }
            });
        }
        status_minute.push(new_status);
    }

    let d1 = distance(
        &status_minute,
        (-1, 0),
        (height, width - 1),
        period,
        (height, width),
        0
    );
    println!("{}", d1);

    let d2 = distance(
        &status_minute,
        (height, width - 1),
        (-1,0),
        period,
        (height, width),
        d1
    );

    let d3 = distance(
        &status_minute,
        (-1, 0),
        (height, width - 1),
        period,
        (height, width),
        d2
    );

    println!("{}", d3);

    

    
    
}
