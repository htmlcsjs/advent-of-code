use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Day;

pub struct DayFour2022;
impl Day for DayFour2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> String {
        let mut count: u32 = 0;
        for (line, r) in input.lines().enumerate() {
            match r {
                Ok(s) => {
                    let mut info: Vec<i32> = Vec::new();
                    for elememnt in s.replace(",", "-").split("-") {
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
                Err(e) => println!("Error readling line {}: {}", line, e),
            }
        }
        count.to_string()
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> String {
        let mut count: u32 = 0;
        for (line, r) in input.lines().enumerate() {
            match r {
                Ok(s) => {
                    let mut info: Vec<i32> = Vec::new();
                    for elememnt in s.replace(",", "-").split("-") {
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
                Err(e) => println!("Error readling line {}: {}", line, e),
            }
        }
        count.to_string()
    }

    fn day(&self) -> i16 {
        4
    }

    fn year(&self) -> i16 {
        2022
    }
}
