use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Write},
    path::Path,
};

use crate::{day_one::DayOne2022, day_two::DayTwo2022};

mod day_one;
mod day_two;

trait Day {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> String;
    fn second_puzzle(&self, input: &mut BufReader<File>) -> String;

    fn day(&self) -> i16;
    fn year(&self) -> i16;
}

struct Day0Year22;
impl Day for Day0Year22 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> String {
        let mut line = String::new();
        input.read_line(&mut line).expect("Failed to read line:");
        line
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> String {
        let mut line = String::new();
        input.read_line(&mut line).expect("Failed to read line:");
        line.trim().to_string()
    }

    fn day(&self) -> i16 {
        0
    }

    fn year(&self) -> i16 {
        2022
    }
}

fn main() {
    let days: Vec<&dyn Day> = vec![&DayOne2022, &DayTwo2022];
    println!("Current puzzles:");
    for (i, day) in days.iter().enumerate() {
        println!("{}: Day {}, Year {}", i + 1, day.day(), day.year());
    }
    print!("What day do you want to run?: ");
    let mut input = String::new();
    stdout().flush().expect("WTF");
    stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    if let Ok(option) = input.trim().parse::<usize>() {
        let mut input = String::new();
        let day = days[option - 1];
        print!("Puzzle one or two?: ");
        stdout().flush().expect("WTF");
        stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        let formatted_input = input.trim();
        if formatted_input == "one" || formatted_input == "1" {
            let what_the_fuck = format!("input/y{}-d{}.txt", day.year(), day.day());
            let path = Path::new(&what_the_fuck);
            let file = match File::open(path) {
                Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
                Ok(file) => file,
            };
            println!(
                "The awnser is\n{}",
                day.first_puzzle(&mut BufReader::new(file))
            );
        } else if formatted_input == "two" || formatted_input == "2" {
            let what_the_fuck = format!("input/y{}-d{}.txt", day.year(), day.day());
            let path = Path::new(&what_the_fuck);
            let file = match File::open(path) {
                Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
                Ok(file) => file,
            };
            println!(
                "The awnser is {}",
                day.second_puzzle(&mut BufReader::new(file))
            );
        } else {
            println!("Invalid option {}", input.trim());
        }
    } else {
        println!("Invalid option {}", input.trim());
    }
}
