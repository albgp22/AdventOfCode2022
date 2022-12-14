use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
extern crate queues;
use eval::{to_value, Expr};

const DEBUG: bool = false;

#[derive(Debug, Clone)]
struct Monkey<T> {
    id: i32,
    false_monkey: i32,
    true_monkey: i32,
    divisible: i32,
    items: Vec<T>,
    operation: String,
}

#[derive(Debug, Clone)]
struct Remainders {
    rem: Vec<i32>,
    div: Vec<i32>,
}

impl Monkey<i32> {
    fn eval(&self, old: i32) -> i32 {
        Expr::new(&self.operation)
            .value("old", old)
            .exec()
            .unwrap()
            .as_i64()
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn test(&self, i: i32) -> bool {
        i % self.divisible == 0
    }
}

impl Monkey<Remainders> {
    fn eval(&mut self, idx: usize) -> i32 {
        self.items[idx].rem = self.items[idx]
            .rem
            .iter()
            .enumerate()
            .map(|(i, x)| {
                Expr::new(&self.operation)
                    .value("old", x)
                    .exec()
                    .unwrap()
                    .as_i64()
                    .unwrap() as i32
                    % self.items[idx].div[i]
            })
            .collect();
        self.items[idx].rem[self.id as usize]
    }
}

pub fn solve1(lines: &Vec<String>) {
    let mut monkeys: Vec<Monkey<i32>> = vec![];

    let mut lines = lines.iter().filter(|l| l.len() > 0).peekable();
    loop {
        let mut m: Monkey<i32> = Monkey {
            id: 0,
            false_monkey: 0,
            true_monkey: 0,
            divisible: 0,
            items: vec![],
            operation: "old + 3".to_string(),
        };
        m.id = lines
            .next()
            .unwrap()
            .split(" ")
            .skip(1)
            .next()
            .unwrap()
            .replace(":", "")
            .parse::<i32>()
            .unwrap();

        let line = lines.next().unwrap().replace("  Starting items: ", "");
        m.items = line
            .split(",")
            .map(|x| x.replace(" ", "").parse::<i32>().unwrap())
            .collect();

        let line = lines.next().unwrap().replace("  Operation: new = ", "");
        m.operation = line.to_string();

        let line = lines.next().unwrap().replace("  Test: divisible by ", "");
        m.divisible = line.parse().unwrap();

        let line = lines
            .next()
            .unwrap()
            .replace("    If true: throw to monkey ", "");
        m.true_monkey = line.parse().unwrap();

        let line = lines
            .next()
            .unwrap()
            .replace("    If false: throw to monkey ", "");
        m.false_monkey = line.parse().unwrap();

        println!("{:?}", &m);
        monkeys.push(m);

        if lines.peek().is_none() {
            break;
        }
    }

    let mut inspections: Vec<i32> = vec![];
    for _ in 0..monkeys.len() {
        inspections.push(0);
    }
    for round in 0..20 {
        let n_monkeys = monkeys.len();
        for i in 0..n_monkeys {
            println!("Monkey {}:", i);
            let mut to_remove = vec![];
            for ni in 0..monkeys[i].items.len() {
                inspections[i] = inspections[i] + 1;
                println!(
                    "  Monkey inspects an item with a worry level of{}",
                    monkeys[i].items[ni]
                );
                println!(
                    "    After modification, worry level is {}",
                    monkeys[i].eval(monkeys[i].items[ni])
                );
                let val = monkeys[i].eval(monkeys[i].items[ni]) / 3;
                println!(
                    "    Monkey gets bored with item. Worry level is divided by 3 to {}",
                    val
                );
                let target_monkey = match monkeys[i].test(val) {
                    true => {
                        println!(
                            "    Current worry level is divisible by {}",
                            monkeys[i].divisible
                        );
                        monkeys[i].true_monkey
                    }
                    false => {
                        println!(
                            "    Current worry level is not divisible by {}",
                            monkeys[i].divisible
                        );
                        monkeys[i].false_monkey
                    }
                };
                println!(
                    "    Item with worry level {} is thrown to monkey {}",
                    val, target_monkey
                );
                monkeys[target_monkey as usize].items.push(val);
                to_remove.push(ni);
            }
            to_remove.iter().rev().for_each(|x| {
                monkeys[i].items.remove(*x);
            })
        }
        println!(
            "After round {}, the monkeys are holding items with these worry levels:",
            round + 1
        );
        for (i, m) in monkeys.iter().enumerate() {
            println!("Monkey {}: {:?}", i, m.items);
        }
        println!("Inspections: {:?}", inspections);
    }
}

fn solve2(lines: &Vec<String>) {
    let mut monkeys: Vec<Monkey<Remainders>> = vec![];
    //let divisors: Vec<i32> = vec![5, 17, 7, 13, 19, 3, 11, 2];
    let divisors: Vec<i32> = vec![5,17,7,13,19,3,11,2];

    let mut lines = lines.iter().filter(|l| l.len() > 0).peekable();
    loop {
        let mut m: Monkey<Remainders> = Monkey {
            id: 0,
            false_monkey: 0,
            true_monkey: 0,
            divisible: 0,
            items: vec![],
            operation: "old + 3".to_string(),
        };
        m.id = lines
            .next()
            .unwrap()
            .split(" ")
            .skip(1)
            .next()
            .unwrap()
            .replace(":", "")
            .parse::<i32>()
            .unwrap();

        let line = lines.next().unwrap().replace("  Starting items: ", "");
        m.items = line
            .split(",")
            .map(|x| x.replace(" ", "").parse::<i32>().unwrap())
            .map(|i| Remainders {
                rem: divisors.clone().iter().map(|d| i % d).collect(),
                div: divisors.clone(),
            })
            .collect();

        let line = lines.next().unwrap().replace("  Operation: new = ", "");
        m.operation = line.to_string();

        let line = lines.next().unwrap().replace("  Test: divisible by ", "");
        m.divisible = line.parse().unwrap();

        let line = lines
            .next()
            .unwrap()
            .replace("    If true: throw to monkey ", "");
        m.true_monkey = line.parse().unwrap();

        let line = lines
            .next()
            .unwrap()
            .replace("    If false: throw to monkey ", "");
        m.false_monkey = line.parse().unwrap();

        println!("{:?}", &m);
        monkeys.push(m);

        if lines.peek().is_none() {
            break;
        }
    }

    let mut inspections: Vec<i32> = vec![];
    for _ in 0..monkeys.len() {
        inspections.push(0);
    }
    for round in 0..10000 {
        let n_monkeys = monkeys.len();
        for i in 0..n_monkeys {
            println!("Monkey {}:", i);
            let mut to_remove = vec![];
            for ni in 0..monkeys[i].items.len() {
                inspections[i] = inspections[i] + 1;
                println!(
                    "  Monkey inspects an item with a worry level of {:?}",
                    monkeys[i].items[ni]
                );
                let val = monkeys[i].eval(ni);
                println!(
                    "    After modification, worry level is {}",
                    val
                );
                let target_monkey = match val {
                    0 => {
                        println!(
                            "    Current worry level is divisible by {}",
                            monkeys[i].divisible
                        );
                        monkeys[i].true_monkey
                    }
                    _ => {
                        println!(
                            "    Current worry level is not divisible by {}",
                            monkeys[i].divisible
                        );
                        monkeys[i].false_monkey
                    }
                };
                println!(
                    "    Item with worry level {} is thrown to monkey {}",
                    val, target_monkey
                );
                let update_val = monkeys[i].items[ni].clone();
                monkeys[target_monkey as usize].items.push(update_val);
                to_remove.push(ni);
            }
            to_remove.iter().rev().for_each(|x| {
                monkeys[i].items.remove(*x);
            })
        }
        println!(
            "After round {}, the monkeys are holding items with these worry levels:",
            round + 1
        );
        for (i, m) in monkeys.iter().enumerate() {
            println!("Monkey {}: {:?}", i, m.items);
        }
        println!("Inspections: {:?}", inspections);
    }
}

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    solve1(&lines);
    solve2(&lines);
}
