use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;

fn update_t(h: &[i32; 2], t: &mut [i32;2]){
    let dh = h[0]-t[0];
    let dv = h[1]-t[1];
    if dh.abs() > 1 || dv.abs() > 1 {
            t[1] += dv.signum();
            t[0] += dh.signum();
    }
}

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut h = [0; 2];
    let mut t = [0; 2];
    let mut visited: HashSet<(i32,i32)> = HashSet::new();

    for line in &lines{
        let mut ls = line.split(" ");
        let direction = ls.next().unwrap();
        let amount = ls.next().unwrap().parse::<i32>().unwrap();
        (0..amount).for_each(
            |_| {
                match direction {
                    "L" => h[0]-=1,
                    "R" => h[0]+=1,
                    "D" => h[1]-=1,
                    "U" => h[1]+=1,
                    _ => panic!(),
                };
                update_t(&h, &mut t);
                visited.insert((t[0],t[1]));
            }
        )
    }
    println!("Visited tail knot: {}", visited.len());

    let mut knots = [[0;2];10];
    let mut visited: HashSet<(i32,i32)> = HashSet::new();

    for line in &lines{
        let mut ls = line.split(" ");
        let direction = ls.next().unwrap();
        let amount = ls.next().unwrap().parse::<i32>().unwrap();
        (0..amount).for_each(
            |_| {
                match direction {
                    "L" => knots[0][0]-=1,
                    "R" => knots[0][0]+=1,
                    "D" => knots[0][1]-=1,
                    "U" => knots[0][1]+=1,
                    _ => panic!(),
                };

                for i in 1..10{
                    update_t(&knots[i-1].clone(), &mut knots[i])
                }
                visited.insert((knots[9][0],knots[9][1]));
            }
        )
    }
    println!("Visited 9th knot: {}", visited.len());

}