use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    println!("Parsing {}:", line);
    let re =
        Regex::new(r"^Sensor at x=(-{0,1}\d+), y=(-{0,1}\d+): closest beacon is at x=(-{0,1}\d+), y=(-{0,1}\d+)$").unwrap();
    let cap = re.captures(line).unwrap();
    (
        (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
        (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
    )
}

fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() > 0)
        .collect();

    let mut sensor_beacon: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    for line in &lines {
        let (a, b) = parse_line(line);
        sensor_beacon.insert(a, b);
    }

    let mut sol1: HashSet<i32> = HashSet::new();
    let mut distances: HashMap<((i32, i32),(i32,i32)), i32> = HashMap::new();

    sensor_beacon.iter().for_each(|(&a, &b)| {
        distances.insert((a,b), distance(a, b));
    });

    const Y: i32 = 2000000;

    for (&sensor, &beacon) in sensor_beacon.iter() {
        let diameter = distance(sensor, beacon);
        if Y > diameter + sensor.1 && Y < sensor.1 - diameter {
            continue;
        }

        let row_dist = if Y > sensor.1 {
            sensor.1 + diameter - Y
        } else {
            Y - sensor.1 + diameter
        };

        for k in (sensor.0 - row_dist)..(sensor.0 + row_dist) {
            sol1.insert(k);
        }
    }

    println!("{}", sol1.len());

    const LIMIT: i32 = 4000000;
    let mut x = -1;
    for y in 0..=LIMIT {
        'outer: while x <= LIMIT {
            x+=1;
            for (&sensor, &beacon) in sensor_beacon.iter() {
                let d = distance(sensor, (x, y));
                let max_d = *distances.get(&(sensor, beacon)).unwrap();
                if d <= max_d {
                    let leap = max_d - (y - sensor.1).abs() + sensor.0 - x;
                    x += leap;
                    continue 'outer;
                }
            }
            println!("Sol2: {}", x * LIMIT + y);
            return;
        }
    }
}
