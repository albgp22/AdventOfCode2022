use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path;
use std::str::FromStr;

fn points_in_between((i, j): (usize, usize), (ii, jj): (usize, usize)) -> Vec<(usize, usize)> {
    //println!("i:{},j:{},ii={},jj={}", i, j, ii, jj);
    let r: Vec<(usize, usize)>;
    if i == ii {
        if j < jj {
            r = (j + 1..jj).map(|jjj| (i, jjj)).collect()
        } else {
            r = (jj + 1..j).map(|jjj| (i, jjj)).collect()
        }
    } else {
        if i < ii {
            r = (i + 1..ii).map(|iii| (iii, j)).collect()
        } else {
            r = (ii + 1..i).map(|iii| (iii, j)).collect()
        }
    }
    //println! {"{:?}", r};
    return r;
}

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() > 0)
        .collect();

    let mut points: HashMap<(usize, usize), i32> = HashMap::new();

    for line in &lines {
        let lm = line.replace(" ", "");
        let mut path_extremes = lm.split("->");
        let mut p = path_extremes
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap());
        let mut p = (p.next().unwrap(), p.next().unwrap());
        points.insert(p, 0);
        for pp in path_extremes {
            let mut pp = pp.split(",").map(|x| x.parse::<usize>().unwrap());
            let pp = (pp.next().unwrap(), pp.next().unwrap());
            points.insert(pp, 0);
            for ppp in points_in_between(p, pp) {
                points.insert(ppp, 0);
            }
            p = pp;
        }
    }
    //println!("{:?}", points);
    let mut counter = 0;
    'outer: loop {
        let mut c = (500, 0);
        counter += 1;
        'inner: loop {
            //println!("{:?}",&c);
            if c.1 == 1000 {
                break 'outer;
            }
            if points.contains_key(&(c.0, c.1 + 1)) {
                if points.contains_key(&(c.0 - 1, c.1 + 1)) {
                    if points.contains_key(&(c.0 + 1, c.1 + 1)) {
                        points.insert(c, 0);
                        break 'inner;
                    } else {
                        c = (c.0 + 1, c.1 + 1);
                    }
                } else {
                    c = (c.0 - 1, c.1 + 1);
                }
            } else {
                c = (c.0, c.1 + 1);
            }
        }
    }
    println!("Number of points: {}", counter - 1);

    let mut points: HashMap<(usize, usize), i32> = HashMap::new();

    for line in &lines {
        let lm = line.replace(" ", "");
        let mut path_extremes = lm.split("->");
        let mut p = path_extremes
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap());
        let mut p = (p.next().unwrap(), p.next().unwrap());
        points.insert(p, 0);
        for pp in path_extremes {
            let mut pp = pp.split(",").map(|x| x.parse::<usize>().unwrap());
            let pp = (pp.next().unwrap(), pp.next().unwrap());
            points.insert(pp, 0);
            for ppp in points_in_between(p, pp) {
                points.insert(ppp, 0);
            }
            p = pp;
        }
    }
    let floor_level: usize = points.keys().into_iter().map(|(_i, j)| *j).max().unwrap() + 2;
    let mut counter = 0;
    while !points.contains_key(&(500, 0)) {
        let mut c = (500, 0);
        counter += 1;
        'inner: loop {
            //println!("{:?}",&c);
            if c.1 + 1 == floor_level {
                points.insert(c, 0);
                break 'inner;
            }
            if points.contains_key(&(c.0, c.1 + 1)) {
                if points.contains_key(&(c.0 - 1, c.1 + 1)) {
                    if points.contains_key(&(c.0 + 1, c.1 + 1)) {
                        points.insert(c, 0);
                        break 'inner;
                    } else {
                        c = (c.0 + 1, c.1 + 1);
                    }
                } else {
                    c = (c.0 - 1, c.1 + 1);
                }
            } else {
                c = (c.0, c.1 + 1);
            }
        }
    }
    println!("Number of points: {}", counter);
}
