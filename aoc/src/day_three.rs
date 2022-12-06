use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::{AOCError, Day};

pub struct DayThree2022;
impl Day for DayThree2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let mut priorities: Vec<usize> = Vec::new();
        let alphabet = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        for r in input.lines() {
            let s = r?;
            let first = &s[0..(s.len() / 2)];
            let second = &s[(s.len() / 2)..];
            let mut shared: char = ' ';
            for c in first.chars() {
                if second.contains(c) {
                    shared = c;
                    break;
                }
            }
            if let Some(p) = alphabet.find(shared) {
                println!("\"{}\" and \"{}\" share \"{}\", with a priority of {}", first, second, shared, p);
                priorities.push(p);
            } else {
                println!("\"{}\" and \"{}\" share \"{}\", with no priority", first, second, shared);
            }
        }
        Ok(priorities.iter().sum::<usize>().to_string())
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let mut priorities: Vec<usize> = Vec::new();
        let alphabet = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let lines: Vec<String> = input.lines().collect::<Result<Vec<_>, io::Error>>()?;
        for group in lines.chunks(3) {
            let (a, b, c) = (group[0].to_owned(), group[1].to_owned(), group[2].to_owned());
            let mut badge: char = ' ';
            for i in a.chars() {
                if b.contains(i) && c.contains(i) {
                    badge = i;
                    break;
                }
            }
            if let Some(p) = alphabet.find(badge) {
                priorities.push(p);
            }
            println!("Group \"{}\", \"{}\", \"{}\" has badge {}", a, b, c, badge);
        }
        Ok(priorities.iter().sum::<usize>().to_string())
    }

    fn day(&self) -> i16 {
        3
    }

    fn year(&self) -> i16 {
        2022
    }
}
