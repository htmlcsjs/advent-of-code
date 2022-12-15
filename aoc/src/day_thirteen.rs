use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use serde_json::{json, Value};

use crate::{AOCError, Day};

pub struct DayThirteen2022;

impl Day for DayThirteen2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let mut pairs = Vec::new();
        let mut ordered = Vec::new();
        for mut i in &input.lines().chunks(3) {
            let lhs: Value = serde_json::from_str(&i.next().unwrap()?)?;
            let rhs: Value = serde_json::from_str(&i.next().unwrap()?)?;
            pairs.push(lhs);
            pairs.push(rhs);
        }
        for (index, i) in (&pairs.iter().chunks(2)).into_iter().enumerate() {
            let (lhs, rhs) = i.collect_tuple().unwrap();
            let result = cmp(lhs, rhs);
            if result == Ordering::Less {
                ordered.push(index + 1);
            }
        }
        Ok(ordered.iter().sum::<usize>().to_string())
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError> {
        let mut packets = Vec::new();
        for mut i in &input.lines().chunks(3) {
            let lhs: Value = serde_json::from_str(&i.next().unwrap()?)?;
            let rhs: Value = serde_json::from_str(&i.next().unwrap()?)?;
            packets.push(lhs);
            packets.push(rhs);
        }
        let dividers = vec![json!([[2]]), json!([[6]])];
        packets.extend(vec![json!([[2]]), json!([[6]])]);
        packets.sort_by(cmp);

        let sus: usize = dividers.iter().map(|x| packets.iter().position(|y| y == x).unwrap() + 1).product();
        Ok(sus.to_string())
    }

    fn day(&self) -> i16 {
        13
    }

    fn year(&self) -> i16 {
        2022
    }
}

fn cmp(lhs: &Value, rhs: &Value) -> Ordering {
    if let Value::Number(l) = lhs {
        if let Value::Number(r) = rhs {
            return l.as_i64().unwrap().cmp(&r.as_i64().unwrap());
        } else if let Value::Array(_) = rhs {
            return cmp(&Value::Array(vec![Value::Number(l.clone())]), rhs);
        }
    } else if let Value::Array(l) = lhs {
        if let Value::Array(r) = rhs {
            for i in 0..l.len().min(r.len()) {
                let sus = cmp(&l[i], &r[i]);
                if sus != Ordering::Equal {
                    return sus;
                }
            }
            return l.len().cmp(&r.len());
        } else if let Value::Number(r) = rhs {
            return cmp(lhs, &Value::Array(vec![Value::Number(r.clone())]));
        }
    }
    Ordering::Equal
}
