use std::{env, fs};

fn part1(input: String) -> String {
    input
        .lines()
        .collect::<Vec<&str>>()
        .split(|line| *line == "")
        .map(|lines| {
            lines
                .iter()
                .map(|line| line.parse::<i32>().expect("Unable to parse number"))
                .sum()
        })
        .max()
        .unwrap_or(0)
        .to_string()
    // let iter = input.lines().peekable();
    // iter.
    // while iter.peek().is_some() {}
}

fn part2(input: String) -> String {
    let mut calories: Vec<i32> = input
        .lines()
        .collect::<Vec<&str>>()
        .split(|line| *line == "")
        .map(|lines| {
            lines
                .iter()
                .map(|line| line.parse::<i32>().expect("Unable to parse number"))
                .sum()
        })
        .collect();
    calories.sort();
    calories.into_iter().rev().take(3).sum::<i32>().to_string()
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
