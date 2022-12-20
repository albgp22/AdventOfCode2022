use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solve(reader: BufReader<File>) {
    let mut nums: Vec<(usize, i64)> = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() > 0)
        .map(|x| x.parse::<i64>().unwrap())
        .enumerate()
        .collect();

    let backup_nums = nums.clone();

    //Part 1
    nums.clone().iter().for_each(|(p, i)| {
        let origin_index = nums.iter().position(|&x| x == (*p, *i)).unwrap();
        let dest_index = (origin_index as i64 + i).rem_euclid(nums.len() as i64 - 1) as usize;
        let origin_element = nums.remove(origin_index);
        nums.insert(dest_index, origin_element);
    });

    let zero_idx = nums.iter().position(|&x| x.1 == 0).unwrap();
    println!(
        "Sol 1: {}",
        vec![1000, 2000, 3000]
            .iter()
            .map(|of| nums.get((zero_idx + of) % nums.len()).unwrap().1)
            .sum::<i64>()
    );

    let mut nums: Vec<(usize, i64)> = backup_nums
        .iter()
        .map(|(i, j)| (*i, j.checked_mul(811589153).unwrap()))
        .collect();
    let numsclone = nums.clone();

    (0..10).for_each(|_| {
        let _ = &numsclone.iter().for_each(|(p, i)| {
            let origin_index = nums.iter().position(|&x| x == (*p, *i)).unwrap();
            let dest_index = usize::try_from(
                (i64::try_from(origin_index).unwrap() + i)
                    .rem_euclid(i64::try_from(nums.len()).unwrap() - 1),
            )
            .unwrap();
            let origin_element = nums.remove(origin_index);
            nums.insert(dest_index, origin_element);
        });
    });

    let zero_idx = nums.iter().position(|&x| x.1 == 0).unwrap();
    println!(
        "Sol 2: {}",
        vec![1000, 2000, 3000]
            .iter()
            .map(|of| nums.get((zero_idx + of) % nums.len()).unwrap().1)
            .sum::<i64>()
    );
}