use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use regex::Regex;

use crate::{AOCError, Day};

pub struct DaySeven2022;
impl Day for DaySeven2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let fs = load_fs(input)?;
        Ok(fs.get_solution(100000).to_string())
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let fs = load_fs(input)?;
        let space_needed = 30000000 - (70000000 - fs.get_size());
        Ok(fs.get_largest(space_needed).into_iter().min().unwrap().to_string())
    }

    fn day(&self) -> i16 {
        7
    }

    fn year(&self) -> i16 {
        2022
    }
}

fn load_fs(input: &mut BufReader<File>) -> Result<FSType, AOCError> {
    let cd_regex = Regex::new("\\$ cd (.+)")?;
    let file_regex = Regex::new("(\\d+) (.+)")?;
    let dir_regex = Regex::new("dir (.+)")?;
    let mut path: Vec<String> = Vec::new();
    let mut fs: FSType = FSType::Dir { children: HashMap::new() };
    for maybe in input.lines() {
        let line = maybe?;
        if line.starts_with('$') {
            if cd_regex.is_match(&line) {
                let s = cd_regex.captures(&line).unwrap();
                match &s[1] {
                    ".." => {
                        path.pop();
                    }
                    "/" => {}
                    _ => path.push(s[1].to_string()),
                }
            } else {
                continue;
            }
        } else if file_regex.is_match(&line) {
            let caps = file_regex.captures(&line).unwrap();
            let mut fs_pos = &mut fs;
            for i in &path {
                if let FSType::Dir { children, .. } = fs_pos {
                    fs_pos =
                        children.get_mut(i).ok_or_else(|| AOCError::LogicError("Error finding path".to_string()))?;
                } else {
                    panic!("Not implmented for files")
                }
            }
            if let FSType::Dir { children, .. } = fs_pos {
                children.insert(caps[2].to_string(), FSType::File { size: caps[1].parse()? });
            }
        } else if dir_regex.is_match(&line) {
            let caps = dir_regex.captures(&line).unwrap();
            let mut fs_pos = &mut fs;
            for i in &path {
                if let FSType::Dir { children, .. } = fs_pos {
                    fs_pos =
                        children.get_mut(i).ok_or_else(|| AOCError::LogicError("Error finding path".to_string()))?;
                } else {
                    panic!("Not implmented for files")
                }
            }
            if let FSType::Dir { children, .. } = fs_pos {
                children.insert(caps[1].to_string(), FSType::Dir { children: HashMap::new() });
            }
        }
    }
    Ok(fs)
}

#[derive(Debug)]
enum FSType {
    File { size: u32 },
    Dir { children: HashMap<String, FSType> },
}

impl FSType {
    fn get_size(&self) -> u32 {
        match self {
            FSType::File { size } => *size,
            FSType::Dir { children } => children.values().into_iter().fold(0, |acc, x| acc + x.get_size()),
        }
    }

    fn get_solution(&self, limit: u32) -> u32 {
        let processed_size = if self.get_size() < limit { self.get_size() } else { 0 };
        match self {
            FSType::File { .. } => processed_size,
            FSType::Dir { children } => children
                .values()
                .into_iter()
                .filter(|x| matches!(x, FSType::Dir { .. }))
                .fold(processed_size, |acc, x| acc + x.get_solution(limit)),
        }
    }

    fn get_largest(&self, smallest: u32) -> Vec<u32> {
        let processed_size = if self.get_size() >= smallest { self.get_size() } else { 0 };
        match self {
            FSType::File { .. } => vec![processed_size],
            FSType::Dir { children } => {
                let mut start = children
                    .values()
                    .filter(|x| matches!(x, FSType::Dir { .. }))
                    .flat_map(|x| x.get_largest(smallest))
                    .filter(|x| x > &0)
                    .collect_vec();
                if processed_size > 0 {
                    start.push(processed_size);
                }
                start
            }
        }
    }
}
