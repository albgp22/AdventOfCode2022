use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum GemType {
    Ore,
    Clay,
    Obs,
    Geo,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Blueprint {
    id: i32,
    costs: HashMap<(GemType, GemType), i32>,
}

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() > 0)
        .collect();

    let blueprints = {
        let mut blueprints: Vec<Blueprint> = vec![];

        for line in &lines {
            let scan = sscanf::sscanf!(line, "Blueprint {i32}: Each ore robot costs {i32} ore. Each clay robot costs {i32} ore. Each obsidian robot costs {i32} ore and {i32} clay. Each geode robot costs {i32} ore and {i32} obsidian.").unwrap();
            blueprints.push(Blueprint {
                id: scan.0,
                costs: HashMap::from([
                    ((GemType::Ore, GemType::Ore), scan.1),
                    ((GemType::Clay, GemType::Ore), scan.2),
                    ((GemType::Obs, GemType::Ore), scan.3),
                    ((GemType::Obs, GemType::Clay), scan.4),
                    ((GemType::Geo, GemType::Ore), scan.5),
                    ((GemType::Geo, GemType::Obs), scan.6),
                ]),
            });
        }

        blueprints
    };

    println!("{:#?}", blueprints);
}
