use std::{env, fs};

fn pprint_containers(containers: &Vec<Vec<char>>) {
    for (i, c) in containers.iter().enumerate() {
        println!("{i}: {}", c.iter().collect::<String>());
    }
    println!();
}

fn parse_input(input: String) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut lines = input.lines();
    let mut container_lines: Vec<&str> =
        lines.by_ref().take_while(|line| !line.is_empty()).collect();

    let num_containers = container_lines
        .pop()
        .unwrap()
        .split_ascii_whitespace()
        .count();

    let mut containers: Vec<Vec<char>> = (0..num_containers).map(|_| Vec::new()).collect();

    for line in container_lines.iter().rev() {
        for i in 0..num_containers {
            let c = line.as_bytes()[i * 4 + 1] as char;
            if !c.is_whitespace() {
                containers[i].push(c);
            }
        }
    }

    let moves: Vec<(usize, usize, usize)> = lines
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|word| word.parse().ok())
                .collect()
        })
        .map(|v: Vec<usize>| (v[0], v[1] - 1, v[2] - 1))
        .collect();

    // pprint_containers(&containers);
    (containers, moves)
}

fn part1(input: String) -> String {
    let (mut containers, moves) = parse_input(input);

    for (num, from, to) in moves {
        // println!("Move {num} from {from} to {to}");
        for _ in 0..num {
            let pack = containers[from].pop().unwrap();
            containers[to].push(pack);
        }
        pprint_containers(&containers);
    }

    containers.into_iter().filter_map(|mut v| v.pop()).collect()
}

fn part2(input: String) -> String {
    let (mut containers, moves) = parse_input(input);

    for (num, from, to) in moves {
        // println!("Move {num} from {from} to {to}");
        let start = containers[from].len() - num;
        let mut all = containers[from].drain(start..).collect();
        containers[to].append(&mut all);
        pprint_containers(&containers);
    }

    containers.into_iter().filter_map(|mut v| v.pop()).collect()
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
