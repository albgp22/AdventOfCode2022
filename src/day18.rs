use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn find_internal(h: &HashSet<(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
    let mut neighbors: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut internal: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut external: HashSet<(i32, i32, i32)> = HashSet::new();
    for n in h {
        for dn in vec![
            (1, 0, 0),
            (-1, 0, 0),
            (0, 0, 1),
            (0, 0, -1),
            (0, 1, 0),
            (0, -1, 0),
        ] {
            let new: (i32, i32, i32) = (n.0 + dn.0, n.1 + dn.1, n.2 + dn.2);
            if !h.contains(&new) {
                neighbors.insert(new);
            }
        }
    }
    const MAX_QUEUE_SIZE: usize = 10000;
    const MAX_ITERATIONS: usize = 10000;
    for n in neighbors {
        if external.contains(&n) {
            continue;
        }
        if !internal.contains(&n) {
            let mut sourr = vec![n];
            let mut iterations = 0;
            let mut to_visit: Vec<(i32, i32, i32)> = vec![n];
            while sourr.len() < MAX_QUEUE_SIZE && iterations < MAX_ITERATIONS && to_visit.len() > 0
            {
                iterations += 1;
                let (vx, vy, vz) = to_visit.pop().unwrap();
                let neighbors: Vec<(i32, i32, i32)> = vec![
                    (1, 0, 0),
                    (-1, 0, 0),
                    (0, 0, 1),
                    (0, 0, -1),
                    (0, 1, 0),
                    (0, -1, 0),
                ]
                .iter()
                .map(|&(x, y, z)| (vx + x, vy + y, vz + z))
                .filter(|x| !sourr.contains(x))
                .filter(|x| !h.contains(x))
                .collect();
                for c in neighbors {
                    sourr.push(c);
                    to_visit.push(c);
                }
            }
            if iterations == MAX_ITERATIONS || to_visit.len() == 0 {
                for c in sourr {
                    internal.insert(c);
                }
            } else {
                for c in sourr {
                    external.insert(c);
                }
            }
        }
    }

    internal
}

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() > 0)
        .collect();

    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();
    for line in &lines {
        cubes.insert(sscanf::sscanf!(line, "{i32},{i32},{i32}").unwrap());
    }

    let mut r = 0;
    for &(x, y, z) in &cubes {
        r += 6;
        for &(x2, y2, z2) in &cubes {
            if (x - x2).abs() + (y - y2).abs() + (z - z2).abs() == 1 {
                r -= 1;
            }
        }
    }

    println!("Sol 1: {}", r);

    let internal = find_internal(&cubes);
    //println!("{}. {:?}", internal.len(), internal);

    let mut r = 0;
    for &(x, y, z) in &cubes {
        r += 6;
        r -= vec![
            (1, 0, 0),
            (-1, 0, 0),
            (0, 0, 1),
            (0, 0, -1),
            (0, 1, 0),
            (0, -1, 0),
        ]
        .iter()
        .map(|&(xx, yy, zz)| (x + xx, y + yy, z + zz))
        .filter(|c| cubes.contains(c) || internal.contains(c))
        .count();
    }

    println!("Sol 2: {}", r);
}