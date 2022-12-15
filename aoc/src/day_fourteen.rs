use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

use itertools::Itertools;

use crate::{AOCError, Day};

pub struct DayFourteen2022;
impl Day for DayFourteen2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let mut grid = create_grid(input)?;

        let mut falling = false;
        let mut count = 0;
        while !falling {
            let mut stopped = false;
            let mut pos = (500, 0);
            while !stopped {
                if pos.1 >= grid.len() - 2 {
                    falling = true;
                    stopped = true;
                } else if grid[pos.1 + 1][pos.0] == ' ' {
                    pos.1 += 1;
                } else if grid[pos.1 + 1][pos.0 - 1] == ' ' {
                    pos.1 += 1;
                    pos.0 -= 1;
                } else if grid[pos.1 + 1][pos.0 + 1] == ' ' {
                    pos.1 += 1;
                    pos.0 += 1;
                } else {
                    stopped = true;
                    grid[pos.1][pos.0] = 'o';
                    count += 1;
                }
            }
        }

        #[cfg(debug_assertions)]
        File::create("input/y2022-d14-p1.txt")?
            .write_all(grid.iter().map(|x| x.iter().join("")).join("\n").as_bytes())?;
        Ok(count.to_string())
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let mut grid = create_grid(input)?;

        let mut falling = false;
        let mut count = 0;
        while !falling {
            let mut stopped = false;
            let mut pos = (500, 0);
            while !stopped {
                if grid[pos.1 + 1][pos.0] == ' ' {
                    pos.1 += 1;
                } else if grid[pos.1 + 1][pos.0 - 1] == ' ' {
                    pos.1 += 1;
                    pos.0 -= 1;
                } else if grid[pos.1 + 1][pos.0 + 1] == ' ' {
                    pos.1 += 1;
                    pos.0 += 1;
                } else {
                    stopped = true;
                    grid[pos.1][pos.0] = 'o';
                    count += 1;
                    if pos == (500, 0) {
                        falling = true;
                    }
                }
            }
        }

        #[cfg(debug_assertions)]
        File::create("input/y2022-d14-p2.txt")?
            .write_all(grid.iter().map(|x| x.iter().join("")).join("\n").as_bytes())?;
        Ok(count.to_string())
    }

    fn day(&self) -> i16 {
        14
    }

    fn year(&self) -> i16 {
        2022
    }
}

#[allow(clippy::unnecessary_unwrap)]
fn create_grid(input: &mut BufReader<File>) -> Result<Vec<Vec<char>>, AOCError> {
    let mut grid: Vec<Vec<char>> = vec![vec![' '; 1000]; 200];
    let mut highest = 0;
    for i in input.lines() {
        for posi in i?
            .split(" -> ")
            .map(|x| x.split_once(',').unwrap())
            .map(|x| {
                let py = x.1.parse::<usize>();
                let px = x.0.parse::<usize>();
                if py.is_ok() && px.is_ok() {
                    let ry = py.unwrap();
                    highest = highest.max(ry);
                    Ok((px.unwrap(), ry))
                } else if let Err(e) = px {
                    Err(AOCError::from(e))
                } else if let Err(e) = py {
                    Err(AOCError::from(e))
                } else {
                    Err(AOCError::LogicError("WHAT".to_string()))
                }
            })
            .collect::<Result<Vec<_>, AOCError>>()?
            .windows(2)
        {
            let (start, end) = (posi[0], posi[1]);
            for y in grid.iter_mut().take(end.1.max(start.1) + 1).skip(end.1.min(start.1)) {
                y[start.0] = '#';
            }
            for x in end.0.min(start.0)..=start.0.max(end.0) {
                grid[end.1][x] = '#';
            }
        }
    }
    grid.truncate(highest + 2);
    grid.push(['#'].repeat(grid[0].len()));

    Ok(grid)
}
