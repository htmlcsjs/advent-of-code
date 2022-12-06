use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use itertools::Itertools;
use regex::Regex;

use crate::{AOCError, Day};

pub struct DayFive2022;
impl Day for DayFive2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        handler(input, |count, source, dest, stack| {
            let range = stack[source - 1].len();
            let mut tmp = stack[source - 1].drain((range - count)..range).collect_vec();
            tmp.reverse();
            stack[dest - 1].append(&mut tmp);
        })
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        handler(input, |count, source, dest, stack| {
            let range = stack[source - 1].len();
            let mut tmp = stack[source - 1].drain((range - count)..range).collect_vec();
            stack[dest - 1].append(&mut tmp);
        })
    }

    fn day(&self) -> i16 {
        5
    }

    fn year(&self) -> i16 {
        2022
    }
}

fn handler<F>(input: &mut BufReader<File>, executor: F) -> Result<String, AOCError>
where
    F: Fn(usize, usize, usize, &mut Vec<Vec<String>>),
{
    let lines: Vec<String> = input.lines().collect::<Result<Vec<_>, io::Error>>()?;
    let breakpoint = lines
        .iter()
        .position(|x| x.is_empty())
        .ok_or_else(|| AOCError::LogicError("Cannot find breaking line".to_string()))?;
    let mut info = lines.split_at(breakpoint).0.to_vec();
    let mut instrs = lines.split_at(breakpoint).1.to_vec();
    let mut stack: Vec<Vec<String>> = Vec::new();
    instrs.remove(0);
    stack.resize(info.pop().expect("wtf").replace(' ', "").len(), Vec::new());
    let sus: Vec<String> = info.iter().map(|x| x.replace("    ", " [_]")).collect();
    println!("{}", sus.join("\n"));
    sus.iter()
        .map(|x| x.replace(' ', ""))
        .map(|x| x.replace('[', ""))
        .map(|x| x.replace(']', ""))
        .enumerate()
        .for_each(|(line, x)| {
            x.chars().enumerate().for_each(|(pos, s)| {
                stack
                    .get_mut(pos)
                    .unwrap_or_else(|| panic!("what. pos: {}, line: {}", pos, line))
                    .insert(0, s.to_string())
            })
        });
    for i in &mut stack {
        i.retain(|x| x != "_");
    }
    let re = Regex::new("move (\\d+) from (\\d+) to (\\d+)")?;
    for (line, instr) in instrs.iter().enumerate() {
        if let Some(caps) = re.captures(instr) {
            let mut invalid = false;
            let mut error: Option<AOCError> = None;
            let possible: Option<(usize, usize, usize)> = caps
                .iter()
                .dropping(1)
                .map(|x| {
                    if let Some(y) = x {
                        let s = y.as_str();
                        match s.parse::<usize>() {
                            Ok(i) => i,
                            Err(e) => {
                                error = Some(e.into());
                                usize::MAX
                            }
                        }
                    } else {
                        invalid = true;
                        usize::MAX
                    }
                })
                .collect_tuple();
            if let Some(e) = error {
                return Err(e);
            }
            if let Some((count, source, dest)) = possible {
                println!("count: {}, source: {}, dest: {}, line \"{}\"", count, source, dest, instr);
                executor(count, source, dest, &mut stack);
            }
        } else {
            println!("Failed to parse line {}", line)
        }
    }
    // I love AI
    // Get the last element of each inner Vec
    let last_elements: Vec<String> =
        stack.iter().map(|inner| inner.last().unwrap_or(&"".to_string()).to_string()).collect();
    // Concatenate the strings
    Ok(last_elements.join(""))
}
