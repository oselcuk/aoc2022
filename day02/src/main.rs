use std::{env, fs};

#[derive(PartialEq, Copy, Clone, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<i32> for Move {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let vals = [Move::Rock, Move::Paper, Move::Scissors];
        for val in vals {
            if value == (val as i32) {
                return Ok(val);
            }
        }
        Err(())
    }
}

impl TryFrom<char> for Move {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(()),
        }
    }
}

fn parse_round_1(round: &str) -> (Move, Move) {
    let chars: Vec<char> = round.chars().collect();
    (chars[0].try_into().unwrap(), chars[2].try_into().unwrap())
}

fn parse_round_2(round: &str) -> (Move, Move) {
    let chars: Vec<char> = round.chars().collect();
    let them: Move = chars[0].try_into().unwrap();
    let us = chars[2];
    (
        them,
        match us {
            'X' => ((them as i32 + 2) % 3).try_into().unwrap(),
            'Y' => them,
            'Z' => ((them as i32 + 1) % 3).try_into().unwrap(),
            _ => panic!(),
        },
    )
}

fn score_round<F>(round: &str, parser: F) -> i32
where
    F: Fn(&str) -> (Move, Move),
{
    let (them, us) = parser(round);
    // println!("Them: {them:?}\tUs: {us:?}");
    use Move::*;
    us as i32
        + 1
        + match (us, them) {
            _ if us == them => 3,

            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 0,
            _ => 6,
        }
}

fn part1(input: String) -> String {
    input
        .lines()
        .map(|round| score_round(round, parse_round_1))
        .sum::<i32>()
        .to_string()
}

fn part2(input: String) -> String {
    input
        .lines()
        .map(|round| score_round(round, parse_round_2))
        .sum::<i32>()
        .to_string()
}

fn main() {
    env::set_current_dir(module_path!()).expect("Unable to change dir to module folder");
    let result1 = part1(fs::read_to_string("input1.txt").expect("Unable to read input 1"));
    println!("Part 1 result:\n\n{result1}\n");
    fs::write("output1.txt", result1).expect("Unable to write output 1");
    let result2 = part2(fs::read_to_string("input2.txt").expect("Unable to read input 2"));
    println!("Part 2 result:\n\n{result2}\n");
    fs::write("output2.txt", result2).expect("Unable to write output 2");
}
