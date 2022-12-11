use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Debug)]
struct Node {
    name: String,
    size: usize,
    child: Vec<Box<Node>>,
}

impl Node {
    fn insert_dir(&mut self, v: &mut Vec<String>) {
        let mut c = self;
        v.drain(0..1);
        //println!("{:#?}", v);
        //println!("{:#?}", &c);
        while v.len() > 1 {
            let key = v.remove(0);
            //println!("{:#?}", &v);
            //println!("{} find...", &key);
            c = c.get_by_name(key).unwrap();
            //println!("Found!")
        }
        //println!("Llego!");
        c.child.push(Box::new(Node {
            name: v.pop().unwrap(),
            size: 0,
            child: vec![],
        }));
        //println!("Salgo!");
    }

    fn insert_file(&mut self, v: &mut Vec<String>, size: usize) {
        let mut c = self;
        v.remove(0);
        while v.len() > 1 {
            c = c.get_by_name(v.remove(0)).unwrap();
        }
        c.child.push(Box::new(Node {
            name: v.pop().unwrap(),
            size: size,
            child: vec![],
        }))
    }

    fn compute_size(&self) -> usize {
        if self.child.is_empty() {
            return self.size;
        }
        return self.child.iter().map(|c| c.compute_size()).sum::<usize>();
    }

    fn compute_solution_size(&self, sizes: &mut Vec<usize>) -> usize {
        if self.child.is_empty() {
            return self.size;
        }

        let mut r = 0;
        for c in &self.child{
            let rr = c.compute_solution_size(sizes); 
            r += rr
        }
        sizes.push(r);
        return r;
    }

    fn compute_solution2_size(&self, sizes: &mut Vec<(String, usize)>) -> usize {
        if self.child.is_empty() {
            return self.size;
        }

        let mut r = 0;
        for c in &self.child{
            let rr = c.compute_solution2_size(sizes); 
            r += rr
        }
        sizes.push((self.name.clone(),r));
        return r;
    }

    fn get_by_name(&mut self, name: String) -> Option<&mut Self> {
        for c in &mut self.child {
            if c.name == name {
                return Some(c);
            }
        }
        None
    }
}

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let re_cd = Regex::new(r"^\$ cd ([^\.]*)$").unwrap();
    let re_cd_root = Regex::new(r"^\$ cd /$").unwrap();
    let re_ls = Regex::new(r"^\$ ls$").unwrap();
    let re_cd_dd = Regex::new(r"^\$ cd \.\.$").unwrap();
    let re_dir = Regex::new(r"^dir (.+)$").unwrap();
    let re_file = Regex::new(r"^(\d+) (.+)$").unwrap();

    let mut file_system = Node {
        name: "/".to_string(),
        size: 0,
        child: vec![],
    };

    let mut curr_path: Vec<String> = vec![];

    for line in lines {
        match 1 {
            _ if re_cd_root.is_match(&line) => {
                curr_path = vec!["/".to_string()];
            }
            _ if re_cd.is_match(&line) => {
                let captures = re_cd.captures(&line).unwrap();
                let dir_name = captures.get(1).unwrap().as_str();

                curr_path.push(dir_name.to_string());
            }
            _ if re_ls.is_match(&line) => {}
            _ if re_cd_dd.is_match(&line) => {
                curr_path.pop();
            }
            _ if re_file.is_match(&line) => {
                let captures = re_file.captures(&line).unwrap();
                let file_name = captures.get(2).unwrap().as_str();
                //println!("Inserting file: {}", &file_name);
                let size = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                //println!("Inserting {}", file_name);
                curr_path.push(file_name.to_string());
                file_system.insert_file(&mut curr_path.clone(), size);
                curr_path.pop();
            }
            _ if re_dir.is_match(&line) => {
                let dir_name = re_dir.captures(&line).unwrap().get(1).unwrap().as_str();
                curr_path.push(dir_name.to_string());
                //println!("Inserting directory: {}", dir_name);
                file_system.insert_dir(&mut curr_path.clone());
                curr_path.pop();
            }
            _ => panic!("{}", &line),
        }
    }

    println!("{:#?}", file_system);
    //println!("{}\n", file_systemcompute_size());
    let mut v = vec!();
    file_system.compute_solution_size(&mut v);
    //println!("{:#?}",&v);
    println!("{}", v.iter().filter(|s| **s<=100000).sum::<usize>());

    let mut v = vec!();
    let free = 70000000 - file_system.compute_size();
    println!("Free space: {}", free);
    let needed = 30000000 - free;
    println!("Needed space: {}", needed);
    file_system.compute_solution2_size(&mut v);
    
    println!(
        "{} with size {}",
        &v
            .iter()
            .filter(|(_,s)| *s>=needed)
            .min_by(|x,y| {(x.1).cmp(&y.1)} )
            .unwrap()
            .0,
        &v
            .iter()
            .filter(|(_,s)| *s>=needed)
            .min_by(|x,y| {(x.1).cmp(&y.1)} )
            .unwrap()
            .1
        )
}
