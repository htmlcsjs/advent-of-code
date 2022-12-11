use std::{
    fs::File,
    io::{self, BufReader, Seek},
    num::ParseIntError,
    path::Path,
    time::Instant,
};

use ::error_macro::ErrorWrapper;
use itertools::Itertools;

use crate::{
    day_eight::DayEight2022, day_five::DayFive2022, day_four::DayFour2022, day_nine::DayNine2022, day_one::DayOne2022,
    day_seven::DaySeven2022, day_six::DaySix2022, day_ten::DayTen2022, day_three::DayThree2022, day_two::DayTwo2022,
};

mod day_eight;
mod day_five;
mod day_four;
mod day_nine;
mod day_one;
mod day_seven;
mod day_six;
mod day_ten;
mod day_three;
mod day_two;
mod error_macro;

trait Day {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError>;
    fn second_puzzle(&self, input: &mut BufReader<File>) -> Result<String, AOCError>;

    fn day(&self) -> i16;
    fn year(&self) -> i16;
}

#[derive(Debug, ErrorWrapper)]
pub enum AOCError {
    IoError(io::Error),
    IntParseError(ParseIntError),
    LogicError(String),
    RegexError(regex::Error),
}

fn main() {
    let days: Vec<&dyn Day> = vec![
        &DayOne2022,
        &DayTwo2022,
        &DayThree2022,
        &DayFour2022,
        &DayFive2022,
        &DaySix2022,
        &DaySeven2022,
        &DayEight2022,
        &DayNine2022,
        &DayTen2022,
    ];
    let mut table =
        vec!["Day", "A Result", "B Result", "A Time", "B Time"].into_iter().map(|x| vec![x.to_string()]).collect_vec();

    for day in days {
        table[0].push(day.day().to_string());

        let raw_path = format!("input/y{}-d{}.txt", day.year(), day.day());
        let path = Path::new(&raw_path);
        let file = match File::open(path) {
            Err(_) => {
                table[1].push(format!("Couldnt read file {}", path.display()));
                table[2].push("<-".to_string());
                table[3].push("  0.000s".to_string());
                table[4].push("  0.000s".to_string());
                continue;
            }
            Ok(file) => file,
        };
        let mut buf_reader = BufReader::new(file);

        let p1start = Instant::now();
        match day.first_puzzle(&mut buf_reader) {
            Ok(a) => table[1].push(a),
            Err(e) => table[1].push(format!("Error: {:?}", e)),
        }
        table[3].push(format!("{:>09.3?}", p1start.elapsed()));
        if let Err(e) = buf_reader.rewind() {
            table[2].push(format!("Error rewinding: {}", e));
            table[4].push("  0.000s".to_string());
        }

        let p2start = Instant::now();
        match day.second_puzzle(&mut buf_reader) {
            Ok(a) => table[2].push(a),
            Err(e) => table[2].push(format!("Error: {:?}", e)),
        }
        table[4].push(format!("{:>09.3?}", p2start.elapsed()));
    }

    let widths =
        table.iter().map(|x| x.iter().max_by(|x, y| x.len().cmp(&y.len())).unwrap()).map(|x| x.len()).collect_vec();
    println!("{}", "╔═".to_string() + &widths.iter().map(|x| "═".repeat(*x)).join("═╦═") + "═╗");
    println!(
        "{}",
        "║ ".to_string() + &table.iter().enumerate().map(|(pos, x)| pad_string(&x[0], widths[pos])).join(" ║ ") + " ║"
    );
    println!("{}", "╠═".to_string() + &widths.iter().map(|x| "═".repeat(*x)).join("═╬═") + "═╣");
    for i in 1..table[0].len() {
        println!(
            "{}",
            "║ ".to_string()
                + &table.iter().enumerate().map(|(pos, x)| pad_string(&x[i], widths[pos])).join(" ║ ")
                + " ║"
        );
    }
    println!("{}", "╚═".to_string() + &widths.iter().map(|x| "═".repeat(*x)).join("═╩═") + "═╝");
}

fn pad_string(input: &String, length: usize) -> String {
    let pad_len = length - input.chars().count();
    input.to_string() + &" ".repeat(pad_len)
}
