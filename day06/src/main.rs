use std::{collections::HashSet, env, fs};

fn solve(input: String, uniques: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(uniques)
        .enumerate()
        .take_while(|(_, v)| HashSet::<char>::from_iter(v.iter().cloned()).len() != uniques)
        .last()
        .unwrap()
        .0
        + uniques
        + 1
}

fn part1(input: String) -> String {
    solve(input, 4).to_string()
}

fn part2(input: String) -> String {
    solve(input, 14).to_string()
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
