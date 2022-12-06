use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{AOCError, Day};

pub struct DayFour2022;
impl Day for DayFour2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let mut count: u32 = 0;
        for r in input.lines() {
            let mut info: Vec<i32> = Vec::new();
            for elememnt in r?.replace(",", "-").split("-") {
                if let Ok(sus) = elememnt.parse::<i32>() {
                    info.push(sus);
                }
            }
            if info.len() < 4 {
                continue;
            }
            let e1: Vec<i32> = (info[0]..=info[1]).collect();
            let e2: Vec<i32> = (info[2]..=info[3]).collect();
            if e1.iter().all(|i| e2.contains(i)) || e2.iter().all(|i| e1.contains(i)) {
                count += 1;
            }
        }
        Ok(count.to_string())
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let mut count: u32 = 0;
        for r in input.lines() {
            let mut info: Vec<i32> = Vec::new();
            for elememnt in r?.replace(",", "-").split("-") {
                if let Ok(sus) = elememnt.parse::<i32>() {
                    info.push(sus);
                }
            }
            if info.len() < 4 {
                continue;
            }
            let e1: Vec<i32> = (info[0]..=info[1]).collect();
            let e2: Vec<i32> = (info[2]..=info[3]).collect();
            if e1.iter().map(|i| e2.contains(i)).fold(false, |acc, i| (acc || i)) {
                count += 1;
            }
        }
        Ok(count.to_string())
    }

    fn day(&self) -> i16 {
        4
    }

    fn year(&self) -> i16 {
        2022
    }
}
