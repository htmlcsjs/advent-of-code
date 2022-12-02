use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Day;

pub struct DayTwo2022;
impl Day for DayTwo2022 {
    fn first_puzzle(&self, input: &mut BufReader<File>) -> String {
        let mut score: u32 = 0;
        for (line, r) in input.lines().enumerate() {
            match r {
                Ok(s) => {
                    if let Some((opponent, player)) = s.split_once(' ') {
                        let plr_shape = u32::from(player.chars().last().expect(".last() dead")) - 87;
                        let opp_shape = u32::from(opponent.chars().last().expect(".last() dead")) - 64;
                        score += plr_shape;
                        let outcome: i32 = self.rps(opp_shape as i32, plr_shape as i32);
                        if outcome == 0 {
                            score += 3;
                        } else if outcome.is_negative() {
                            continue;
                        } else if outcome.is_positive() {
                            score += 6;
                        }
                    } else {
                        println!("Line {} is misformatted (\"{}\")", line, s);
                    }
                }
                Err(e) => println!("Error readling line {}: {}", line, e),
            }
        }
        score.to_string()
    }

    fn second_puzzle(&self, input: &mut BufReader<File>) -> String {
        let mut score: u32 = 0;
        for (line, r) in input.lines().enumerate() {
            match r {
                Ok(s) => {
                    if let Some((opponent, dec)) = s.split_once(' ') {
                        let outcome = (u32::from(dec.chars().last().expect(".last() dead")) as i32) - 89;
                        let opp_shape = u32::from(opponent.chars().last().expect(".last() dead")) - 64;
                        let plr_shape = self.skuffed_rps(opp_shape, outcome);
                        score += plr_shape;
                        if outcome == 0 {
                            println!("We drew, adding {} to score", 3);
                            score += 3;
                        } else if outcome.is_negative() {
                            println!("We lost, gaining no score");
                            continue;
                        } else if outcome.is_positive() {
                            println!("We won, adding {} to score", 6);
                            score += 6;
                        }
                    } else {
                        println!("Line {} is misformatted (\"{}\")", line, s);
                    }
                }
                Err(e) => println!("Error readling line {}: {}", line, e),
            }
        }
        score.to_string()
    }

    fn day(&self) -> i16 {
        2
    }

    fn year(&self) -> i16 {
        2022
    }
}

impl DayTwo2022 {
    // rock = 1, paper = 2, scissors = 3
    fn rps(&self, opp: i32, plr: i32) -> i32 {
        if (opp == 1 && plr == 3) || (opp == 2 && plr == 1) || (opp == 3 && plr == 2) {
            return -1;
        } else if (plr == 1 && opp == 3) || (plr == 2 && opp == 1) || (plr == 3 && opp == 2) {
            return 1;
        }
        0
    }

    fn skuffed_rps(&self, opp: u32, outcome: i32) -> u32 {
        if outcome == 1 {
            return (opp % 3) + 1;
        } else if outcome == -1 {
            if opp == 1 {
                return 3;
            } else if opp == 2 {
                return 1;
            } else if opp == 3 {
                return 2;
            }
        }
        opp
    }
}
