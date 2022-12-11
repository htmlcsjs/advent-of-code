use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

use crate::{AOCError, Day};

pub struct DayEleven2022;
impl Day for DayEleven2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        monkey_business(input, 3, 20)
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        monkey_business(input, -1, 10000)
    }

    fn day(&self) -> i16 {
        11
    }

    fn year(&self) -> i16 {
        2022
    }
}

fn monkey_business(input: &mut BufReader<File>, mut relef_const: i64, rounds: i64) -> Result<String, AOCError> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for (id, i) in (&input.lines().chunks(7)).into_iter().enumerate() {
        let mut raw = Vec::new();
        for line in i {
            raw.push(line?);
        }
        let op_info = raw[2][23..].split(' ').collect_vec();
        monkeys.push(Monkey {
            items: raw[1][18..].split(", ").map(|x| x.parse::<i64>().expect("Fail to parse num")).collect_vec(),
            op: match op_info[0] {
                "*" => match op_info[1] {
                    "old" => Ops::Sqr,
                    num => Ops::Mul(num.parse()?),
                },
                "+" => Ops::Add(op_info[1].parse()?),
                op => return Err(AOCError::LogicError(format!("Cannot read operaton {}", op))),
            },
            check: raw[3][21..].parse()?,
            dest: (raw[4][29..].parse()?, raw[5][30..].parse()?),
            id,
            count: 0,
        });
    }
    if relef_const == -1 {
        relef_const = monkeys.iter().map(|x| x.check).product();
    }
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].clone();
            if relef_const == 3 {
                monkeys = monkey.turn(monkeys, relef_const);
            } else {
                monkeys = monkey.turn2(monkeys, relef_const);
            }
        }
    }
    let counts = monkeys.iter().map(|x| x.count).collect_vec();
    let biggest = counts.iter().max().ok_or_else(|| AOCError::LogicError("No Max".to_string()))?;
    let second =
        counts.iter().filter(|x| x != &biggest).max().ok_or_else(|| AOCError::LogicError("No Second".to_string()))?;
    Ok((biggest * second).to_string())
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    op: Ops,
    check: i64,
    dest: (usize, usize),
    id: usize,
    count: i64,
}

impl Monkey {
    fn turn(&mut self, mut friends: Vec<Monkey>, relef_const: i64) -> Vec<Monkey> {
        for i in &self.items {
            let new = self.op.execute(*i) / relef_const;
            if new % self.check == 0 {
                friends[self.dest.0].items.push(new);
            } else {
                friends[self.dest.1].items.push(new);
            }
            self.count += 1;
        }
        self.items.clear();
        friends[self.id] = self.clone();
        friends
    }
    fn turn2(&mut self, mut friends: Vec<Monkey>, relef_const: i64) -> Vec<Monkey> {
        for i in &self.items {
            let new = self.op.execute(*i) % relef_const;
            if new % self.check == 0 {
                friends[self.dest.0].items.push(new);
            } else {
                friends[self.dest.1].items.push(new);
            }
            self.count += 1;
        }
        self.items.clear();
        friends[self.id] = self.clone();
        friends
    }
}

#[derive(Debug, Clone)]
enum Ops {
    Add(i64),
    Mul(i64),
    Sqr,
}

impl Ops {
    fn execute(&self, x: i64) -> i64 {
        match self {
            Ops::Add(y) => x + y,
            Ops::Mul(y) => x * y,
            Ops::Sqr => x * x,
        }
    }
}
