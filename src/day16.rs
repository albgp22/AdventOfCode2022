use genetic_algorithm::strategy::evolve::{self, prelude::*};
use genetic_algorithm::compete::{CompeteTournament, CompeteDispatch};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(PartialEq, Clone, Debug)]
enum ActionType {
    Move(usize),
    Activate,
    None,
}

#[derive(Clone, Debug)]
struct GetFlow{
    adj: Vec<Vec<bool>>,
    flows: Vec<u16>,
    impossible_penalty: u16,
    initial_node: usize,
}

impl Fitness for GetFlow {
    type Genotype = DiscreteGenotype<ActionType>;

    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let mut r: i32 = 0;
        let mut activated = vec![];
        let mut curr_node = self.initial_node;
        for (i, action) in chromosome.genes.iter().enumerate(){
            match action {
                ActionType::Activate => {
                    if !activated.contains(&curr_node){
                        activated.push(curr_node);
                        r += (30-i as i32)*(self.flows[curr_node] as i32);
                    }else{
                        r = r - self.impossible_penalty as i32;
                    }
                },
                ActionType::Move(j) => {
                    //println!("{},{}", curr_node, *j);
                    if !self.adj[curr_node][*j] {
                        r = r - self.impossible_penalty as i32;
                    }
                    curr_node = *j;
                }
                ActionType::None => {},
            }
        }
        Some(r as FitnessValue)
    }
}

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
    let mut flows: Vec<u16> = vec![];

    for (i, line) in lines.iter().enumerate() {
        let cap = re.captures(line).unwrap();
        let mut cap_iter = cap.iter().skip(1);

        let origin = cap_iter.next().unwrap().unwrap().as_str();
        let rate = cap_iter
            .next()
            .unwrap()
            .unwrap()
            .as_str()
            .parse::<u16>()
            .unwrap();
        let dests: Vec<&str> = cap_iter
            .filter(|x| x.is_some())
            .map(|x| x.unwrap().as_str())
            .collect();

        println!("Origin: {}", origin);
        println!("Rate: {}", rate);
        println!("Dests: {:?}", dests);

        adj_string.insert(origin, dests);
        flows.push(rate);
        indices.insert(origin, i);
    }

    let n = flows.len();
    let mut adj = vec![vec![false; n]; n];
    for (origin, dests) in adj_string.iter() {
        for d in dests {
            adj[*indices.get(origin).unwrap()][*indices.get(d).unwrap()] = true;
        }
    }
    drop(adj_string);

    // the search space
    let genotype = DiscreteGenotype::builder()
        .with_genes_size(30)
        .with_allele_list({
            let mut x: Vec<ActionType> = (0..n).map(|i| ActionType::Move(i)).collect();
            x.push(ActionType::Activate);
            x.push(ActionType::None);
            x
        })
        .build()
        .unwrap();

    println!("{}", genotype);

    let mut rng = rand::thread_rng();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(10000)
        .with_fitness(GetFlow{adj: adj, flows: flows, impossible_penalty: 1000, initial_node: *indices.get("AA").unwrap()})
        .with_mutate(MutateOnce(0.5))
        .with_crossover(CrossoverUniform(true))
        .with_compete(CompeteElite)
        .with_max_stale_generations(100000)
        .with_fitness_ordering(FitnessOrdering::Maximize)
        .call(&mut rng)
        .unwrap();

    
    println!("{}", evolve);
    println!("{:?}",indices);
}
