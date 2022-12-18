use itertools::Itertools;
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

fn circle(s: &(i32, i32), distance: i32) -> HashSet<(i32, i32)> {
    let mut r = HashSet::new();
    for i in 0..(distance + 1) {
        r.insert((s.0 + i, s.1 + distance - i));
        r.insert((s.0 - i, s.1 + distance - i));
        r.insert((s.0 + i, s.1 - distance + i));
        r.insert((s.0 - i, s.1 - distance + i));
    }
    r
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
    let mut distances: HashMap<((i32, i32), (i32, i32)), i32> = HashMap::new();

    sensor_beacon.iter().for_each(|(&a, &b)| {
        distances.insert((a, b), distance(a, b));
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

    let mut sensors_distances: HashMap<(i32, i32), i32> = HashMap::new();

    for ((a, b), d) in distances.iter() {
        sensors_distances.insert(*a, *d);
    }

    let sensors_by_distance = sensors_distances.keys().sorted_by(|a, b| {
        sensors_distances
            .get(&a)
            .unwrap()
            .cmp(sensors_distances.get(&b).unwrap())
    });

    const LIMIT: i32 = 4000000;
    for sensor in sensors_by_distance {
        let d = *sensors_distances.get(&sensor).unwrap();
        println!("Looking at a distance {} from {:?}", d, &sensor);
        for max_distance in vec![d, d + 1] {
            let c = circle(&sensor, max_distance);
            //println!("{:?}", &c);
            for point in c {
                if point.0 <= 0 || point.1 <= 0 || point.0 >= LIMIT || point.1 >= LIMIT {
                    continue;
                }
                if distances.iter().all(|((s, _b), d)| {
                    /*println!(
                        "Distance: {}, sensor_distance: {}, point: {:?}, sensor: {:?}",
                        distance(*s, point),
                        *d,
                        &point,
                        &s,
                    );*/
                    distance(*s, point) > *d
                }) {
                    println!(
                        "Solution 2: {:?}: {}",
                        (point.0, point.1),
                        point.0 * 4000000 + point.1
                    );
                    return;
                }
            }
        }
    }
}
