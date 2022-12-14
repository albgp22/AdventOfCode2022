use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
extern crate queues;
use queues::*;

const N: usize = 164;
const M: usize = 42;

struct Map {
    cells: [[i32; N]; M],
    start: (usize, usize),
    end: (usize, usize),
}

fn add_i32_to_usize(a: usize, b: i32) -> Option<usize> {
    match b.signum() {
        1 => Some(a + (b as usize)),
        -1 => {
            let b = -b;
            let b = b as usize;
            a.checked_sub(b).or(None)
        }
        0 => Some(a),
        _ => panic!(),
    }
}

fn c_to_i32(c: &char) -> i32 {
    *c as i32
}

impl Map {
    pub fn get_adjacent(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut r = vec![];
        for di in -1..=1 {
            for dj in -1..=1 {
                if di != 0 && dj != 0 {
                    continue;
                }
                let ii = add_i32_to_usize(i, di);
                let jj = add_i32_to_usize(j, dj);
                if ii.is_none() || jj.is_none() {
                    continue;
                }
                let ii = ii.unwrap();
                let jj = jj.unwrap();
                //println!("{},{}",ii,jj);

                if ii < M {
                    if jj < N {
                        if dj != 0 || di != 0 {
                            if self.cells[ii][jj] <= self.cells[i][j] + 1 {
                                r.push((ii, jj));
                            }
                        }
                    }
                }
            }
        }
        r
    }

    pub fn bfs(&self, start: (usize,usize)) -> i32{
        let mut q: Queue<(usize, usize)> = queue![];
        let mut v: Vec<(usize, usize)> = vec![];
        let mut p: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        q.add(start).unwrap();
        v.push(start);

        while q.size() != 0 {
            let c = q.remove().unwrap();
            if c == self.end {
                break;
            }
            for neighbor in self.get_adjacent(c.0, c.1) {
                if !v.contains(&neighbor) {
                    p.insert(neighbor, c);
                    v.push(neighbor);
                    q.add(neighbor).unwrap();
                }
            }
        }

        // Get distance
        let mut a = self.end;
        let mut c = 0;
        //println!("{:?}", p);
        if !p.contains_key(&self.end){
            return i32::MAX;
        }

        while a != start {
            //println!("{:?}", a);
            c += 1;
            a = *p.get(&a).unwrap();
        }
        c
    }

    pub fn bfs_mod(&self, start: (usize,usize)) -> (i32,Vec<(usize,usize)>){
        let mut q: Queue<(usize, usize)> = queue![];
        let mut v: Vec<(usize, usize)> = vec![];
        let mut p: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        q.add(start).unwrap();
        v.push(start);

        while q.size() != 0 {
            let c = q.remove().unwrap();
            if c == self.end {
                break;
            }
            for neighbor in self.get_adjacent(c.0, c.1) {
                if !v.contains(&neighbor) {
                    p.insert(neighbor, c);
                    v.push(neighbor);
                    q.add(neighbor).unwrap();
                }
            }
        }

        // Get distance
        let mut a = self.end;
        let mut c = 0;
        let mut a_s: Vec<(usize,usize)> = vec![];
        let mut min = i32::MAX;
        //println!("{:?}", p);
        if !p.contains_key(&self.end){
            return (i32::MAX, vec![]);
        }

        loop {
            //println!("{:?}", a);
            if self.cells[a.0][a.1] == c_to_i32(&'a'){
                if min > c {
                    min=c;
                }
                a_s.push(a);
            }
            a = *p.get(&a).unwrap();
            if a==start{break;}

            c += 1;
        }
        (min, a_s)
    }
}

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut map = Map {
        cells: [[0; N]; M],
        start: (0, 0),
        end: (0, 0),
    };
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    map.start = (i, j);
                    map.cells[i][j] = c_to_i32(&'a');
                }
                'E' => {
                    map.end = (i, j);
                    map.cells[i][j] = c_to_i32(&'z');
                }
                _ => {
                    map.cells[i][j] = c_to_i32(&c);
                }
            }
        }
    }

    // println!("{:?}", map.cells);
    println!("End: {:?}", map.end);
    println!(
        "Neighbors for end: {:?}",
        map.get_adjacent(map.end.0, map.end.1)
    );
    println!(
        "Neighbors for Start: {:?}",
        map.get_adjacent(map.start.0, map.start.1)
    );
    println!("Start: {:?}", map.start);

    println!("Minimum distance is {}", map.bfs(map.start));

    let mut a_s: Vec<(usize,usize)> = vec![];
    for i in 0..M{
        for j in 0..N{
            if map.cells[i][j] == c_to_i32(&'a'){
                a_s.push((i,j));
            }
        }
    }

    println!("{} possible starting points. Calculating...", a_s.len());
    let mut min = i32::MAX;
    while !a_s.is_empty(){
        //println!("Current size: {}. Current minimum: {}", a_s.len(), min);
        let c = a_s.pop().unwrap();
        let (d,aa_s) = map.bfs_mod(c);
        if d<min{
            min = d
        }
        for cc in aa_s{
            if a_s.iter().position(|x| {*x == cc}).is_some(){
                a_s.remove(a_s.iter().position(|x| {*x == cc}).unwrap());
            }
        }
    }

    println!("{}",min);

}
