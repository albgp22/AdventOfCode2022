use std::cmp::min;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum ValueType {
    Integer,
    List,
}

#[derive(Clone, Debug)]
struct Value {
    t: ValueType,
    v1: Vec<Box<Value>>,
    v2: i32,
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        //println!("Comparing {:#?} and {:#?}",self,other);
        match (self.t, other.t) {
            (ValueType::Integer, ValueType::Integer) => self.v2.partial_cmp(&other.v2),
            (ValueType::Integer, ValueType::List) => Self {
                t: ValueType::List,
                v1: vec![Box::new(Value {
                    t: ValueType::Integer,
                    v1: vec![],
                    v2: self.v2,
                })],
                v2: 0,
            }
            .partial_cmp(&other),
            (ValueType::List, ValueType::Integer) => self.partial_cmp(&Self {
                t: ValueType::List,
                v1: vec![Box::new(Value {
                    t: ValueType::Integer,
                    v1: vec![],
                    v2: other.v2,
                })],
                v2: 0,
            }),
            (ValueType::List, ValueType::List) => {
                let common = min(self.v1.len(), other.v1.len());
                for idx in 0..common {
                    match self.v1[idx].partial_cmp(&other.v1[idx]) {
                        None => return None,
                        Some(Ordering::Less) => return Some(Ordering::Less),
                        Some(Ordering::Greater) => return Some(Ordering::Greater),
                        _ => {}
                    }
                }
                if common < self.v1.len() {
                    Some(Ordering::Greater)
                } else if common < other.v1.len() {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Equal)
                }
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self.t, other.t) {
            (ValueType::Integer, ValueType::Integer) => self.v2.eq(&other.v2),
            (ValueType::Integer, ValueType::List) => Self {
                t: ValueType::List,
                v1: vec![Box::new(Value {
                    t: ValueType::Integer,
                    v1: vec![],
                    v2: self.v2,
                })],
                v2: 0,
            }
            .eq(&other),
            (ValueType::List, ValueType::Integer) => self.eq(&Self {
                t: ValueType::List,
                v1: vec![Box::new(Value {
                    t: ValueType::Integer,
                    v1: vec![],
                    v2: other.v2,
                })],
                v2: 0,
            }),
            (ValueType::List, ValueType::List) => self.v1.eq(&other.v1),
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
struct ParseValueError {}

impl FromStr for Value {
    type Err = ParseValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //println!("{}", &s);
        let l = s.chars().collect::<Vec<char>>().len();
        // Is integer
        if s.parse::<i32>().is_ok() {
            return Ok(Self {
                t: ValueType::Integer,
                v1: vec![],
                v2: s.parse().unwrap(),
            });
        }
        if l == 2 {
            return Ok(Self {
                t: ValueType::List,
                v1: vec![],
                v2: 0,
            });
        }
        //Get rid of braces
        let mut mbz = 0;
        let mut last = 0;
        let s = &s[1..l - 1];
        let mut v = vec![];
        for (i, c) in s.chars().enumerate() {
            match c {
                '[' => mbz += 1,
                ']' => mbz -= 1,
                ',' => {
                    if mbz == 0 {
                        v.push(Box::new(Value::from_str(&s[last..i])?));
                        last = i + 1;
                    }
                }
                _ => {}
            }
        }
        v.push(Box::new(Value::from_str(&s[last..])?));
        Ok(Value {
            t: ValueType::List,
            v1: v,
            v2: 0,
        })
    }
}

pub fn solve(reader: BufReader<File>) {
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() > 0)
        .collect();
    let mut c = 0;
    assert!(lines.len() % 2 == 0);
    for i in 0..lines.len() {
        if i % 2 == 1 {
            continue;
        }
        let a = Value::from_str(&lines[i]).unwrap();
        //println!("a:{:#?}",a);
        let b = Value::from_str(&lines[i + 1]).unwrap();
        //println!("b:{:#?}",b);
        if a <= b {
            //println!("{}", i);
            c += i / 2 + 1;
        }
    }
    println!("Count: {}", c);

    let mut parsed_input: Vec<(&str, Value)> = lines
        .iter()
        .map(|l| (l.as_str(), Value::from_str(&l).unwrap()))
        .collect();
    parsed_input.push((&"[[2]]", Value::from_str(&"[[2]]").unwrap()));
    parsed_input.push((&"[[6]]", Value::from_str(&"[[6]]").unwrap()));

    parsed_input.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());
    let idx_1 = parsed_input.iter().position(|x| x.0 == "[[2]]").unwrap();
    let idx_2 = parsed_input.iter().position(|x| x.0 == "[[6]]").unwrap();

    println!("Solution: {}", (idx_1 + 1) * (idx_2 + 1));
}
