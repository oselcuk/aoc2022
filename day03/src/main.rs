use std::{collections::HashSet, env, fs};

fn histogram(sack: &str) -> HashSet<char> {
    sack.chars().collect()
}

fn get_commons(sacks: &[&str]) -> char {
    sacks
        .into_iter()
        .map(|sack| sack.chars().collect::<HashSet<char>>())
        .reduce(|l, r| l.intersection(&r).cloned().collect())
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
    // sacks
    //     .into_iter()
    //     .map(|sack| HashSet::from_iter(sack.chars()))
    //     .reduce(|left, right| HashSet::from_iter(left.intersection(&right).cloned().collect()));
    // 'a'
}

fn get_common(sack: &str) -> char {
    let (left, right) = sack.split_at(sack.len() / 2);
    get_commons(&[left, right])
}

fn prio_code(c: char) -> i32 {
    c as i32 + 1
        - if c.is_lowercase() {
            'a' as i32
        } else {
            'A' as i32 - 26
        }
}

fn part1(input: String) -> String {
    input
        .lines()
        .map(get_common)
        .map(prio_code)
        .sum::<i32>()
        .to_string()
}

fn part2(input: String) -> String {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(get_commons)
        .map(prio_code)
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
