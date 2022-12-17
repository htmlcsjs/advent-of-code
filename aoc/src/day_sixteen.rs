use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

use itertools::Itertools;
use regex::Regex;

use crate::{AOCError, Day};

pub struct DaySixteen2022;
impl Day for DaySixteen2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let valve_regex = Regex::new("Valve ([A-Z]+) has flow rate=(\\d+); tunnels? leads? to valves? ([A-Z, ]+)")?;
        let mut valves: HashMap<String, Valve> = HashMap::new();

        for line in input.lines().collect::<Result<Vec<String>, io::Error>>()? {
            if let Some(caps) = valve_regex.captures(&line) {
                valves.insert(
                    caps[1].to_string(),
                    Valve {
                        rate: caps[2].parse()?,
                        neighbours: caps[3].split(", ").map(|s| s.to_string()).collect_vec(),
                    },
                );
            }
        }

        match dfs(&mut valves, "AA".to_string(), 1, &"AA".to_string(), 0, 0, &mut HashMap::new(), &Vec::new()) {
            Some(result) => Ok(result.0.to_string()),
            None => Err(AOCError::LogicError("what".to_string())),
        }
    }

    // DOES NOT WORK WITH EXAMPLE
    fn second_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let valve_regex = Regex::new("Valve ([A-Z]+) has flow rate=(\\d+); tunnels? leads? to valves? ([A-Z, ]+)")?;
        let mut valves: HashMap<String, Valve> = HashMap::new();

        for line in input.lines().collect::<Result<Vec<String>, io::Error>>()? {
            if let Some(caps) = valve_regex.captures(&line) {
                valves.insert(
                    caps[1].to_string(),
                    Valve {
                        rate: caps[2].parse()?,
                        neighbours: caps[3].split(", ").map(|s| s.to_string()).collect_vec(),
                    },
                );
            }
        }

        let me =
            dfs(&mut valves, "AA".to_string(), 5, &"AA".to_string(), 0, 0, &mut HashMap::new(), &Vec::new()).unwrap();
        let elephant = dfs(&mut valves, "AA".to_string(), 5, &"AA".to_string(), 0, 0, &mut HashMap::new(), &me.2);

        Ok((me.0 + elephant.unwrap().0).to_string())
    }

    fn day(&self) -> i16 {
        16
    }

    fn year(&self) -> i16 {
        2022
    }
}

// cannot count to 3
struct Valve {
    rate: i64,
    neighbours: Vec<String>,
}

#[allow(clippy::too_many_arguments)]
fn dfs(
    valves: &mut HashMap<String, Valve>,
    v: String,
    min: i8,
    path: &String,
    score: i64,
    rate: i64,
    cache: &mut HashMap<(i8, String, i64), i64>,
    turned: &Vec<String>,
) -> Option<DFSResult> {
    if min > 30 {
        return Some(DFSResult(score, path.to_string(), turned.to_vec()));
    }
    let key = (min, v.to_string(), rate);
    if let Some(val) = cache.get(&key) {
        if val >= &score {
            return None;
        }
    }
    cache.insert(key, score);

    let cur = valves.get(&v).unwrap();

    let open = if cur.rate == 0 || turned.contains(&v) {
        None
    } else {
        let new_rate = rate + cur.rate;
        let mut new_turned = turned.iter().cloned().collect_vec();
        new_turned.push(v.clone());
        dfs(valves, v.clone(), min + 1, &format!("{} -> Turn", path), score + rate, new_rate, cache, &new_turned)
    };

    let neighbours = valves[&v].neighbours.iter().cloned().collect_vec();
    let moved = neighbours
        .iter()
        .filter_map(|x| {
            dfs(valves, x.to_owned(), min + 1, &format!("{} -> {}", path, x), score + rate, rate, cache, turned)
        })
        .max();

    moved.max(open)
}

#[derive(Debug, PartialEq, Eq)]
struct DFSResult(i64, String, Vec<String>);

impl PartialOrd for DFSResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Ord for DFSResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
