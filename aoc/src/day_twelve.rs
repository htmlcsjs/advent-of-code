use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use serde::Serialize;

use crate::{AOCError, Day};

type Grid = Vec<Vec<Node>>;
type Pos = (usize, usize);

pub struct DayTwelve2022;
impl Day for DayTwelve2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let (grid, start, end) = bfs(input)?;

        let (path, status) = find_path(end, &grid, start);

        #[cfg(debug_assertions)]
        serde_json::to_writer(File::create("input/out.json")?, &Debug { grid, path: path.to_vec() })?;

        match status {
            Status::Found => Ok(path.len().to_string()),
            Status::NoPath => Err(AOCError::LogicError("Could not find path".to_string())),
            Status::Searchng => Ok("Advancement unlocked: How did we get here".to_string()),
        }
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let (grid, _, end) = bfs(input)?;

        let mut paths = Vec::new();

        for s in grid.iter().flatten().filter(|x| x.height == 1) {
            let (path, status) = find_path(end, &grid, (s.x, s.y));
            match status {
                Status::Found => paths.push(path),
                Status::NoPath => continue,
                Status::Searchng => panic!("Advancement unlocked: How did we get here"),
            }
        }
        let path = paths.iter().min_by(|x, y| x.len().cmp(&y.len()));

        match path {
            Some(path) => {
                #[cfg(debug_assertions)]
                serde_json::to_writer(File::create("input/out.json")?, &Debug { grid, path: path.to_vec() })?;
                Ok(path.len().to_string())
            }
            None => Err(AOCError::LogicError("Could not find path".to_string())),
        }
    }

    fn day(&self) -> i16 {
        12
    }

    fn year(&self) -> i16 {
        2022
    }
}

fn find_path(end: (usize, usize), grid: &[Vec<Node>], start: (usize, usize)) -> (Vec<(usize, usize)>, Status) {
    let mut path = vec![end];
    let mut status = Status::Searchng;
    while status == Status::Searchng {
        let last = path.last().unwrap();
        let next = grid[last.1][last.0].parent;
        if let Some(pos) = next {
            if pos == start {
                status = Status::Found;
            } else {
                path.push(pos);
            }
        } else {
            status = Status::NoPath
        }
    }
    (path, status)
}

fn bfs(input: &mut BufReader<File>) -> Result<(Grid, Pos, Pos), AOCError> {
    let (mut grid, start, end) = parse_graph(input)?;
    let mut q = VecDeque::new();
    grid[start.1][start.0].explored = true;
    grid[start.1][start.0].parent = None;
    q.push_back(start);
    while !q.is_empty() {
        let v_pos = q.pop_front().unwrap();

        // i love the rust borrow checker
        let neighbours = grid[v_pos.1][v_pos.0].neighbours.to_vec();
        let neighbours_filted = neighbours.iter().filter(|x| !grid[x.1][x.0].explored).collect_vec();
        for w_pos in neighbours_filted {
            grid[w_pos.1][w_pos.0].explored = true;
            grid[w_pos.1][w_pos.0].parent = Some(v_pos);
            q.push_back(*w_pos);
        }
    }
    Ok((grid, start, end))
}

fn parse_graph(input: &mut BufReader<File>) -> Result<(Grid, Pos, Pos), AOCError> {
    let mut grid: Grid = Vec::new();
    let mut end = (0, 0);
    let mut start = (0, 0);
    for (y, i) in input.lines().enumerate() {
        grid.push(
            i?.chars()
                .enumerate()
                .map(|(x, c)| Node {
                    x,
                    y,
                    neighbours: Vec::new(),
                    height: if c == 'S' {
                        start = (x, y);
                        1
                    } else if c == 'E' {
                        end = (x, y);
                        26
                    } else {
                        u32::from(c) - 96
                    },
                    explored: false,
                    parent: None,
                })
                .collect_vec(),
        )
    }
    let height = grid.len();
    let width = grid[0].len();
    let heights = grid.iter().map(|x| x.iter().map(|y| y.height).collect_vec()).collect_vec();
    for i in &mut grid.iter_mut().flatten() {
        if i.x > 0 && (heights[i.y][i.x - 1] <= i.height + 1) {
            i.neighbours.push((i.x - 1, i.y));
        }
        if i.y > 0 && (heights[i.y - 1][i.x] <= i.height + 1) {
            i.neighbours.push((i.x, i.y - 1));
        }
        if i.x + 1 < width && (heights[i.y][i.x + 1] <= i.height + 1) {
            i.neighbours.push((i.x + 1, i.y));
        }
        if i.y + 1 < height && (heights[i.y + 1][i.x] <= i.height + 1) {
            i.neighbours.push((i.x, i.y + 1));
        }
    }
    Ok((grid, start, end))
}

#[derive(Serialize, Clone)]
struct Node {
    x: usize,
    y: usize,
    neighbours: Vec<Pos>,
    height: u32,
    explored: bool,
    parent: Option<Pos>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Status {
    Found,
    NoPath,
    Searchng,
}

#[cfg(debug_assertions)]
#[derive(Serialize, Clone)]
struct Debug {
    grid: Grid,
    path: Vec<Pos>,
}
