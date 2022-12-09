use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use itertools::Itertools;

use crate::{AOCError, Day};

pub struct DayEight2022;
impl Day for DayEight2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let grid = load_grid(input.lines().collect_vec())?;
        let mut count = 0;
        for x in 0..grid.len() {
            let row = &grid[x];
            for y in 0..row.len() {
                let val = row[y];
                if row[(y + 1)..].iter().max().unwrap_or(&-1) < &val
                    || row[..y].iter().max().unwrap_or(&-1) < &val
                    || grid[(x + 1)..].iter().map(|x| x[y]).max().unwrap_or(-1) < val
                    || grid[..x].iter().map(|x| x[y]).max().unwrap_or(-1) < val
                {
                    count += 1;
                }
            }
        }
        Ok(count.to_string())
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let grid = load_grid(input.lines().collect_vec())?;
        let mut scores = Vec::new();
        for x in 0..grid.len() {
            let row = &grid[x];
            for y in 0..row.len() {
                let val = row[y];
                let mut left = 0;
                for i in row[..y].iter().rev() {
                    left += 1;
                    if i >= &val {
                        break;
                    }
                }
                let mut right = 0;
                for i in &row[(y + 1)..] {
                    right += 1;
                    if i >= &val {
                        break;
                    }
                }
                let mut up = 0;
                for i in grid[..x].iter().map(|x| x[y]).rev() {
                    up += 1;
                    if i >= val {
                        break;
                    }
                }
                let mut down = 0;
                for i in grid[(x + 1)..].iter().map(|x| x[y]) {
                    down += 1;
                    if i >= val {
                        break;
                    }
                }
                scores.push(left * right * up * down);
            }
        }
        Ok(scores
            .iter()
            .max()
            .ok_or_else(|| AOCError::LogicError("Failed to find max, no scores in vec".to_string()))?
            .to_string())
    }

    fn day(&self) -> i16 {
        8
    }

    fn year(&self) -> i16 {
        2022
    }
}

fn load_grid(lines: Vec<Result<String, io::Error>>) -> Result<Vec<Vec<i8>>, AOCError> {
    let mut grid = Vec::new();
    for i in lines {
        let line = i?;
        grid.push(line.chars().map(|x| x.to_string().parse::<i8>().expect("Parse error")).collect_vec());
    }
    Ok(grid)
}
