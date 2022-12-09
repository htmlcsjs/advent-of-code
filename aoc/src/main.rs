use std::{
    fs::File,
    io::{self, stdin, stdout, BufReader, Write},
    num::ParseIntError,
    path::Path,
};

use ::error_macro::ErrorWrapper;

use crate::{
    day_five::DayFive2022, day_four::DayFour2022, day_one::DayOne2022, day_seven::DaySeven2022, day_six::DaySix2022,
    day_three::DayThree2022, day_two::DayTwo2022,
};

mod day_five;
mod day_four;
mod day_one;
mod day_seven;
mod day_six;
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
    let days: Vec<&dyn Day> =
        vec![&DayOne2022, &DayTwo2022, &DayThree2022, &DayFour2022, &DayFive2022, &DaySix2022, &DaySeven2022];
    println!("Current puzzles:");
    for (i, day) in days.iter().enumerate() {
        println!("{}: Day {}, Year {}", i + 1, day.day(), day.year());
    }
    print!("What day do you want to run?: ");
    let mut input = String::new();
    stdout().flush().expect("WTF");
    stdin().read_line(&mut input).expect("error: unable to read user input");
    if let Ok(option) = input.trim().parse::<usize>() {
        let mut input = String::new();
        let day = days[option - 1];
        print!("Puzzle one or two?: ");
        stdout().flush().expect("WTF");
        stdin().read_line(&mut input).expect("error: unable to read user input");
        let formatted_input = input.trim();
        if formatted_input == "one" || formatted_input == "1" {
            let what_the_fuck = format!("input/y{}-d{}.txt", day.year(), day.day());
            let path = Path::new(&what_the_fuck);
            let file = match File::open(path) {
                Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
                Ok(file) => file,
            };
            match day.first_puzzle(&mut BufReader::new(file)) {
                Ok(s) => println!("The awnser is {}", s),
                Err(e) => println!("There was an error executing Day {} puzzle 1:\n{:?}", day.day(), e),
            }
        } else if formatted_input == "two" || formatted_input == "2" {
            let what_the_fuck = format!("input/y{}-d{}.txt", day.year(), day.day());
            let path = Path::new(&what_the_fuck);
            let file = match File::open(path) {
                Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
                Ok(file) => file,
            };
            match day.second_puzzle(&mut BufReader::new(file)) {
                Ok(s) => println!("The awnser is {}", s),
                Err(e) => println!("There was an error executing Day {} puzzle 2:\n{:?}", day.day(), e),
            }
        } else {
            println!("Invalid option {}", input.trim());
        }
    } else {
        println!("Invalid option {}", input.trim());
    }
}
