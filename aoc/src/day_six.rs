use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

use crate::{AOCError, Day};

pub struct DaySix2022;

impl Day for DaySix2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let mut line = String::new();
        let size = 4;
        input.read_line(&mut line)?;
        get_packet_loc(line, size)
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let mut line = String::new();
        let size = 14;
        input.read_line(&mut line)?;
        get_packet_loc(line, size)
    }

    fn day(&self) -> i16 {
        6
    }

    fn year(&self) -> i16 {
        2022
    }
}

fn get_packet_loc(line: String, size: usize) -> Result<String, AOCError> {
    line.chars()
        .collect_vec()
        .windows(size)
        .enumerate()
        .find_map(|(pos, x)| if x.iter().duplicates().next().is_none() { Some((pos + size).to_string()) } else { None })
        .ok_or_else(|| AOCError::LogicError("Failed to find packet start marker".to_string()))
}
