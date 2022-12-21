use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn compute_with_string(
    name: String,
    ops: &mut HashMap<String, String>,
    computed: &mut HashSet<String>,
    values: &mut HashMap<String, i64>,
) -> (i64, String) {
    if values.get_key_value(name.as_str()).is_some() {
        (
            *values.get_key_value(name.as_str()).unwrap().1,
            format!("{}", *values.get_key_value(name.as_str()).unwrap().1).to_string(),
        )
    } else {
        let op = ops.get_key_value(name.as_str()).unwrap().1.clone();
        let mut op = op.split(" ");
        let key1 = op.next().unwrap().trim().to_string();
        let math_op = op.next().unwrap().trim();
        let key2 = op.next().unwrap().trim().to_string();
        let val = match math_op {
            "+" => {
                let val1 = compute_with_string(key1, ops, computed, values);
                let val2 = compute_with_string(key2, ops, computed, values);

                if name != "root" { (
                    val1.0.checked_add(val2.0).unwrap(),
                    format!("({}{}{})", val1.1, math_op, val2.1).to_string(),
                )}else{(
                    val1.0.checked_add(val2.0).unwrap(),
                    format!("{}={}", val1.1, val2.1).to_string(),
                )

                }
            }
            "-" => {
                let val1 = compute_with_string(key1, ops, computed, values);
                let val2 = compute_with_string(key2, ops, computed, values);
                (
                    val1.0.checked_sub(val2.0).unwrap(),
                    format!("({}{}{})", val1.1, math_op, val2.1).to_string(),
                )
            }
            "*" => {
                let val1 = compute_with_string(key1, ops, computed, values);
                let val2 = compute_with_string(key2, ops, computed, values);
                (
                    val1.0.checked_mul(val2.0).unwrap(),
                    format!("({}{}{})", val1.1, math_op, val2.1).to_string(),
                )
            }
            "/" => {
                let val1 = compute_with_string(key1, ops, computed, values);
                let val2 = compute_with_string(key2, ops, computed, values);
                (
                    val1.0.checked_div(val2.0).unwrap(),
                    format!("({}{}{})", val1.1, math_op, val2.1).to_string(),
                )
            }
            _ => panic!("Operation not supported {}", math_op),
        };
        values.insert(name, val.0);
        val
    }
}

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() > 0)
        .collect();

    let mut ops: HashMap<String, String> = HashMap::new();
    let mut computed: HashSet<String> = HashSet::new();
    let mut values: HashMap<String, i64> = HashMap::new();

    for line in &lines {
        let mut parts = line.split(":");
        let name = parts.next().unwrap().trim();
        let op = parts.next().unwrap().trim();
        if op.parse::<i64>().is_ok() {
            computed.insert(name.to_string());
            values.insert(name.to_string(), op.parse::<i64>().unwrap());
        } else {
            ops.insert(name.to_string(), op.trim().to_string());
        }
    }

    let mut values_backup = values.clone();

    println!(
        "Result: {}",
        compute_with_string("root".to_string(), &mut ops, &mut computed, &mut values).0
    );

    //This returns a maxima-solvable equation
    println!(
        "Formula: \nalgsys([{}],[X]);",
        compute_with_string("root".to_string(), &mut ops, &mut computed, &mut values_backup).1.replace("1789", "X")
    );

    //Solution: 3555057453229

}
