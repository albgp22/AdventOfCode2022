use std::fs::File;
use std::io::{prelude::*, BufReader};
use itertools::iproduct;

const N: usize = 99;

fn can_be_seen(mat: &[[u32; N]; N], i: usize, j: usize) -> bool {
    let mut impos = 0;
    for ii in 0..i {
        if mat[ii][j] >= mat[i][j] {
            impos += 1;
            break;
        }
    }
    for ii in i + 1..N {
        if mat[ii][j] >= mat[i][j] {
            impos += 1;
            break;
        }
    }
    for jj in 0..j {
        if mat[i][jj] >= mat[i][j] {
            impos += 1;
            break;
        }
    }
    for jj in j + 1..N {
        if mat[i][jj] >= mat[i][j] {
            impos += 1;
            break;
        }
    }
    impos != 4
}

fn scenic_score(mat: &[[u32; N]; N], i: usize, j: usize) -> i32 {
    let mut sc = [0; 4];
    for ii in (0..i).rev() {
        sc[0] += 1;
        // println!("ij: {},{}",ii,j);
        if mat[ii][j] >= mat[i][j] {
            break;
        }
    }
    for ii in i + 1..N {
        sc[1] += 1;
        // println!("ij: {},{}",ii,j);
        if mat[ii][j] >= mat[i][j] {
            break;
        }
    }
    for jj in (0..j).rev() {
        sc[2] += 1;
        // println!("ij: {},{}",i,jj);
        if mat[i][jj] >= mat[i][j] {
            break;
        }
    }
    for jj in j + 1..N {
        // println!("ij: {},{}",i,jj);
        sc[3] += 1;
        if mat[i][jj] >= mat[i][j] {
            break;
        }
    }
    sc.into_iter().product()
}

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut mat: [[u32; N]; N] = [[0; N]; N];
    let mut num_lines = 0;

    lines.iter().enumerate().for_each(|(i, l)| {
        for (j, c) in l.chars().enumerate() {
            mat[i][j] = c.to_digit(10).unwrap();
        }
        num_lines = i;
    });

    let mut res = 0;
    res += 4 * (N - 1);

    for i in 1..(N - 1) {
        for j in 1..(N - 1) {
            if can_be_seen(&mat, i, j) {
                res += 1
            }
        }
    }
    println!("{:?}", mat);
    println!("Result: {}", res);
    //println!("{}", scenic_score(&mat, 3, 2));
    println!(
        "Max scenic score: {}",
        iproduct!((0..N),(0..N))
            .map(|(i, j)| scenic_score(&mat, i, j))
            .max()
            .unwrap()
    );
}
