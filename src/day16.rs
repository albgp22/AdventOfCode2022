use regex::Regex;
use std::collections::{HashMap};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_flow(l: &str) -> i32 {
    let re =
        Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels{0,1} leads{0,1} to valves{0,1} (?:([A-Z]{2}), )*([A-Z]{2})$").unwrap();
    let cap = re.captures(l).unwrap();
    cap.iter()
        .skip(2)
        .next()
        .unwrap()
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

fn distance(adjacency: &HashMap<usize, Vec<usize>>, i: usize, j: usize) -> i32 {
    if adjacency.get(&i).unwrap().contains(&j) {
        1
    } else {
        i32::MAX / 2
    }
}

fn visit(
    v: usize,
    budget: i32,
    state: u32,
    flow: i32,
    r: &mut HashMap<u32, i32>,
    n: usize,
    distances: &Vec<Vec<i32>>,
    flows: &HashMap<usize, i32>,
) {
    let prev_state_value = *r.get(&state).or(Some(&0)).unwrap();
    r.insert(state, prev_state_value.max(flow));
    for u in 0..n {
        if *flows.get(&u).unwrap() == 0{ continue;}
        let new_budget = budget - distances[v][u] - 1;
        if ((1 << u & state) != 0) || (new_budget <= 0) {
            continue;
        }
        visit(
            u,
            new_budget,
            state | (1 << u),
            flow + new_budget * flows.get(&u).unwrap(),
            r,
            n,
            distances,
            flows,
        );
    }
}

pub fn solve(reader: BufReader<File>) {
    let lines = {
        let mut lines: Vec<String> = reader
            .lines()
            .map(|l| l.unwrap())
            .filter(|l| l.len() > 0)
            .collect();
        lines.sort_by(|a, b| get_flow(b).cmp(&get_flow(a)));
        lines
    };

    let re =
        Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels{0,1} leads{0,1} to valves{0,1} (?:([A-Z]{2}), )*([A-Z]{2})$").unwrap();

    let mut adj_string: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut indices: HashMap<&str, usize> = HashMap::new();
    let mut flows: Vec<i32> = vec![];

    for (i, line) in lines.iter().enumerate() {
        let cap = re.captures(line).unwrap();
        let mut cap_iter = cap.iter().skip(1);

        let origin = cap_iter.next().unwrap().unwrap().as_str();
        let rate = cap_iter
            .next()
            .unwrap()
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        let dests: Vec<&str> = cap_iter
            .filter(|x| x.is_some())
            .map(|x| x.unwrap().as_str())
            .collect();

        adj_string.insert(origin, dests);
        flows.push(rate);
        indices.insert(origin, i);
    }

    let adjacency = {
        let mut r: HashMap<usize, Vec<usize>> = HashMap::new();
        adj_string.iter().for_each(|(origin, dests)| {
            r.insert(
                *indices.get(origin).unwrap(),
                dests.iter().map(|i| *indices.get(i).unwrap()).collect(),
            );
        });
        r
    };

    let flows = {
        let mut r: HashMap<usize, i32> = HashMap::new();
        flows.iter().enumerate().for_each(|(origin, flow)| {
            r.insert(origin, *flow);
        });
        r
    };

    let n = indices.iter().len();
    let mut distances = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                distances[i][j] = 0;
            } else {
                distances[i][j] = distance(&adjacency, i, j);
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                distances[i][j] = distances[i][j].min(distances[i][k] + distances[k][j]);
            }
        }
    }

    // Part 1
    let mut result = HashMap::new();
    visit(
        *indices.get("AA").unwrap(),
        30,
        0,
        0,
        &mut result,
        n,
        &distances,
        &flows,
    );
    println!("{:?}", result.iter().map(|(_k, v)| v).max().unwrap());

    //Part 2
    let mut result = HashMap::new();
    visit(
        *indices.get("AA").unwrap(),
        26,
        0,
        0,
        &mut result,
        n,
        &distances,
        &flows,
    );

    println!(
        "{}",
        &result
            .iter()
            .map(|(k, v)| {
                result
                    .iter()
                    .filter(|(k2, _v2)| (*k & **k2) != 0)
                    .map(|(_k2, v2)| *v2)
                    .max()
                    .or(Some(0))
                    .unwrap()
                    + v
            })
            .max()
            .unwrap()
    )
}
