use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

use regex::Regex;

use crate::{AOCError, Day};

type Pos = (i64, i64);
pub struct DayFifteen2022;
impl Day for DayFifteen2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let line_regex = Regex::new("Sensor at x=(-?\\d+), y=(-?\\d+): closest beacon is at x=(-?\\d+), y=(-?\\d+)")?;
        let mut devices = Vec::new();
        for line in input.lines().collect::<Result<Vec<String>, io::Error>>()? {
            if let Some(caps) = line_regex.captures(&line) {
                devices.push(Sensor::new((caps[1].parse()?, caps[2].parse()?), (caps[3].parse()?, caps[4].parse()?)));
            }
        }

        const Y: i64 = 10;

        let occupied: HashSet<Pos> = devices.iter().flat_map(|x| [x.beacon, x.pos]).collect();
        let max = devices.iter().map(|x| x.pos.0.max(x.beacon.0)).max().unwrap();
        let min = devices.iter().map(|x| x.pos.0.min(x.beacon.0)).min().unwrap();
        let range = devices.iter().map(|x| x.distance).max().unwrap();
        let mut points = 0;
        for x in (min - range)..=(max + range) {
            if occupied.contains(&(x, Y)) {
                continue;
            }
            if devices.iter().any(|z| {
                let other_dist = (z.pos.0 - x).abs() + (z.pos.1 - Y).abs();
                z.distance >= other_dist
            }) {
                points += 1;
            }
        }

        Ok(points.to_string())
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let line_regex = Regex::new("Sensor at x=(-?\\d+), y=(-?\\d+): closest beacon is at x=(-?\\d+), y=(-?\\d+)")?;
        let mut devices = Vec::new();
        for line in input.lines().collect::<Result<Vec<String>, io::Error>>()? {
            if let Some(caps) = line_regex.captures(&line) {
                devices.push(Sensor::new((caps[1].parse()?, caps[2].parse()?), (caps[3].parse()?, caps[4].parse()?)));
            }
        }

        const MIN: Pos = (0, 0);
        const MAX: Pos = (4000000, 4000000);
        if let Some(pos) = find_unseen(&devices, MIN, MAX) {
            Ok((pos.0 * 4000000 + pos.1).to_string())
        } else {
            Err(AOCError::LogicError("Failed to find unseen point".to_string()))
        }
    }

    fn day(&self) -> i16 {
        15
    }

    fn year(&self) -> i16 {
        2022
    }
}

fn find_unseen(devices: &[Sensor], min: Pos, max: Pos) -> Option<Pos> {
    let mut stack: Vec<(Pos, Pos)> = vec![(min, max)];
    while let Some((min, max)) = stack.pop() {
        if min == max {
            if devices.iter().all(|z| {
                let other_dist = (z.pos.0 - min.0).abs() + (z.pos.1 - min.1).abs();
                z.distance < other_dist
            }) {
                return Some(min);
            }
        } else {
            let mid = ((min.0 + max.0) / 2, (min.1 + max.1) / 2);
            let quads = [
                (min, mid),
                ((mid.0 + 1, min.1), (max.0, mid.1)),
                ((min.0, mid.1 + 1), (mid.0, max.1)),
                ((mid.0 + 1, mid.1 + 1), max),
            ];
            for quad in quads {
                if quad.0 .0 > quad.1 .0 || quad.0 .1 > quad.1 .1 {
                    continue;
                } else if devices.iter().all(|z| z.contains_unseen(quad.0, quad.1)) {
                    stack.push(quad);
                }
            }
        }
    }

    None
}

struct Sensor {
    pos: Pos,
    beacon: Pos,
    distance: i64,
}

impl Sensor {
    fn new(pos: Pos, beacon: Pos) -> Self {
        Self { pos, beacon, distance: (pos.0 - beacon.0).abs() + (pos.1 - beacon.1).abs() }
    }

    fn contains_unseen(&self, min: Pos, max: Pos) -> bool {
        let quads = [min, (min.0, max.1), max, (max.0, min.1)];
        quads.iter().map(|z| (z.0 - self.pos.0).abs() + (z.1 - self.pos.1).abs()).max().unwrap() > self.distance
    }
}
