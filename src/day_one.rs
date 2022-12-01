use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Day;

pub struct DayOne2022;
impl Day for DayOne2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> String {
        let elves = self.parse_elves(input);
        elves[0].to_string()
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> String {
        let mut elves = self.parse_elves(input);
        elves.truncate(3);
        elves.iter().sum::<u64>().to_string()
    }

    fn day(&self) -> i16 {
        1
    }

    fn year(&self) -> i16 {
        2022
    }
}

impl DayOne2022 {
    fn parse_elves(&self, input: &mut BufReader<File>) -> Vec<u64> {
        let mut elves: Vec<u64> = Vec::new();
        let mut buffer: u64 = 0;
        for (line, r) in input.lines().enumerate() {
            match r {
                Ok(s) => {
                    if !s.is_empty() {
                        match s.parse::<u64>() {
                            Ok(i) => buffer += i,
                            Err(e) => println!("Error parsing `{}` as u64: {}", s, e),
                        }
                    } else {
                        elves.push(buffer);
                        buffer = 0;
                    }
                }
                Err(e) => println!("Error readling line {}: {}", line, e),
            }
        }
        elves.sort();
        elves.reverse();
        elves
    }
}
