use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

use crate::{AOCError, Day};

pub struct DayTen2022;
impl Day for DayTen2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let mut iter = input.lines().peekable();
        let mut cycle: i64 = 0;
        let mut wait = 0;
        let mut instr = "".to_string();
        let mut x: i64 = 1;
        let addx = Regex::new("addx ([-\\d]+)")?;
        let mut strengths: Vec<i64> = Vec::new();

        while iter.peek().is_some() {
            cycle += 1;
            if cycle % 40 == 20 {
                strengths.push(x * cycle);
            }
            if wait == 0 {
                instr = iter.next().unwrap()?;
            }
            if addx.is_match(&instr) {
                if wait == 0 {
                    wait = 1;
                } else if wait == 1 {
                    x += addx.captures(&instr).expect("No captures in addx inst")[1].parse::<i64>()?;
                    wait -= 1;
                }
            } else if instr == "noop" {
                wait = 0;
            }
        }
        Ok(strengths.iter().sum::<i64>().to_string())
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let mut iter = input.lines().peekable();
        let mut cycle: i64 = 0;
        let mut end_cycle = 0;
        let mut x: i64 = 1;
        let mut inc: i64 = 0;
        let addx = Regex::new("addx ([-\\d]+)")?;
        let mut rendered = "".to_string();

        while iter.peek().is_some() {
            cycle += 1;
            let pixel = (cycle - 1) % 40;
            if pixel == 0 && cycle != 1 {
                rendered += "\n"
            }
            if ((pixel - 1)..=(pixel + 1)).contains(&x) {
                rendered += "▒";
            } else {
                rendered += " ";
            }
            if end_cycle == cycle {
                x += inc;
            }
            if end_cycle < cycle {
                let instr = iter.next().unwrap()?;

                if addx.is_match(&instr) {
                    inc = addx.captures(&instr).expect("No captures in addx inst")[1].parse::<i64>()?;
                    end_cycle = cycle + 1;
                }
            }
        }
        println!("{}", rendered);
        Ok("See above".to_string())
    }

    fn day(&self) -> i16 {
        10
    }

    fn year(&self) -> i16 {
        2022
    }
}
