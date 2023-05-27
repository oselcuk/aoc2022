use std::{collections::HashSet, env, fs};

fn parse_line(line: &str) -> ((i32, i32), i32) {
    let (l, r) = line.split_once(':').unwrap();
    fn parse_part(part: &str) -> (i32, i32) {
        let (l, r) = part.split_once('=').unwrap().1.split_once(',').unwrap();
        let r = r.split_once('=').unwrap().1;
        (l.parse().unwrap(), r.parse().unwrap())
    }
    let (l, r) = (parse_part(l), parse_part(r));
    (l, man_dist(l, r))
}

fn parse_input(input: String) -> Vec<((i32, i32), i32)> {
    input.lines().map(parse_line).collect()
}

fn man_dist(x: (i32, i32), y: (i32, i32)) -> i32 {
    (x.0 - y.0).abs() + (x.1 - y.1).abs()
}

#[derive(Debug, Clone, Copy)]
enum Interval {
    B(i32),
    E(i32),
}
use Interval::*;
fn get_n(i: &Interval) -> i32 {
    match i {
        B(n) => *n,
        E(n) => *n,
    }
}

fn part1(input: String) -> String {
    let readings = parse_input(input);
    let r = 2_000_000;
    let mut seen = Vec::<Interval>::new();
    for (sensor, dist) in readings.iter() {
        let dist = dist - (r - sensor.1).abs();
        if dist >= 0 {
            seen.push(B(sensor.0 - dist));
            seen.push(E(sensor.0 + dist + 1));
        }
    }
    let extras = readings
        .iter()
        .filter(|(x, _)| x.1 == r)
        .map(|((x, _), _)| *x)
        .collect::<HashSet<i32>>()
        .len() as i32;
    seen.sort_by_key(get_n);
    let mut depth = 0;
    let mut count = 0;
    let mut last = 0;
    for i in seen {
        let n = get_n(&i);
        if depth > 0 {
            count += n - last;
        }
        last = n;
        depth += match i {
            B(_) => 1,
            E(_) => -1,
        }
    }
    (count - extras).to_string()
}

fn part2(input: String) -> String {
    let readings = parse_input(input);
    let limit = 4000000;
    for r in 0..limit {
        let mut seen = Vec::<Interval>::new();
        for (sensor, dist) in readings.iter() {
            let dist = dist - (r - sensor.1).abs();
            if dist >= 0 {
                seen.push(B(sensor.0 - dist));
                seen.push(E(sensor.0 + dist + 1));
            }
        }
        seen.sort_by_key(get_n);
        let mut depth = 0;
        let mut last = 0;
        for i in seen {
            let n = get_n(&i);
            let len = n - last;
            if depth == 0 && len > 0 {
                println!("x: {}, y: {}", n - 1, r);
                return (4000000u64 * (n as u64 - 1) + r as u64).to_string();
            }
            last = n;
            depth += match i {
                B(_) => 1,
                E(_) => -1,
            }
        }
    }
    "".to_string()
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
