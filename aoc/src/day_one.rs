use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{AOCError, Day};

pub struct DayOne2022;
impl Day for DayOne2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let elves = self.parse_elves(input)?;
        Ok(elves[0].to_string())
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let mut elves = self.parse_elves(input)?;
        elves.truncate(3);
        Ok(elves.iter().sum::<u64>().to_string())
    }

    fn day(&self) -> i16 {
        1
    }

    fn year(&self) -> i16 {
        2022
    }
}

impl DayOne2022 {
    fn parse_elves(&self, input: &mut BufReader<File>) -> Result<Vec<u64>, AOCError> {
        let mut elves: Vec<u64> = Vec::new();
        let mut buffer: u64 = 0;
        for r in input.lines() {
            let s = r?;
            if !s.is_empty() {
                buffer += s.parse::<u64>()?;
            } else {
                elves.push(buffer);
                buffer = 0;
            }
        }
        elves.sort();
        elves.reverse();
        Ok(elves)
    }
}
