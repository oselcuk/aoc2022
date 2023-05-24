use std::{env, fs};

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    fn parse_interval(interval: &str) -> (i32, i32) {
        let (l, r) = interval.split_once("-").unwrap();
        (l.parse().unwrap(), r.parse().unwrap())
    }
    let (l, r) = line.split_once(",").unwrap();
    (parse_interval(l), parse_interval(r))
}

fn contains(((l1, r1), (l2, r2)): &((i32, i32), (i32, i32))) -> bool {
    (l1 <= l2 && r1 >= r2) || (l1 >= l2 && r1 <= r2)
}

fn overlaps(((l1, r1), (l2, r2)): &((i32, i32), (i32, i32))) -> bool {
    !((l1 < l2 && r1 < l2) || (l1 > r2 && r1 > r2))
}

fn part1(input: String) -> String {
    input
        .lines()
        .map(parse_line)
        .filter(contains)
        .count()
        .to_string()
}

fn part2(input: String) -> String {
    input
        .lines()
        .map(parse_line)
        .filter(overlaps)
        .count()
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
