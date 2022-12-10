use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{AOCError, Day};

pub struct DayNine2022;
type Pos = (i32, i32);
impl Day for DayNine2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        simulate_rope(input, &mut vec![(0, 0); 2])
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        simulate_rope(input, &mut vec![(0, 0); 10])
    }

    fn day(&self) -> i16 {
        9
    }

    fn year(&self) -> i16 {
        2022
    }
}

fn simulate_rope(input: &mut BufReader<File>, rope: &mut Vec<Pos>) -> Result<String, AOCError> {
    let mut visted: HashSet<Pos> = HashSet::new();
    for i in input.lines() {
        let (op, count) = process_line(i?)?;
        let dir = match op.as_str() {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => (0, 0),
        };
        for _ in 0..count {
            rope[0].0 += dir.0;
            rope[0].1 += dir.1;
            for i in 1..rope.len() {
                rope[i] = move_segment(&rope[i], &rope[i - 1]);
            }
            visted.insert(*rope.last().ok_or_else(|| AOCError::LogicError("Rope is empty".to_string()))?);
        }
    }
    Ok(visted.len().to_string())
}

fn process_line(line: String) -> Result<(String, i32), AOCError> {
    let mut binding = line.split_once(' ');
    let (op, count) = binding.get_or_insert(("0", "0"));
    Ok((op.to_string(), count.parse()?))
}

fn move_segment(tail: &Pos, target: &Pos) -> Pos {
    let dx = tail.0 - target.0;
    let dy = tail.1 - target.1;
    if (dx == 2 || dx == -2) && (dy == 2 || dy == -2) {
        (target.0 + dx.clamp(-1, 1), target.1 + dy.clamp(-1, 1))
    } else if dx == 2 || dx == -2 {
        (target.0 + dx.clamp(-1, 1), target.1)
    } else if dy == 2 || dy == -2 {
        (target.0, target.1 + dy.clamp(-1, 1))
    } else {
        *tail // already adjacent
    }
}
