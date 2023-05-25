use std::{env, fs};

fn input_to_stream<'a>(input: &'a String) -> impl Iterator<Item = (i32, i32)> + 'a {
    input
        .lines()
        .scan(1, |reg, line| {
            if let Some((_, x)) = line.split_once(" ") {
                let ret = *reg;
                *reg += x.parse::<i32>().unwrap();
                Some(vec![ret, *reg])
            } else {
                Some(vec![*reg])
            }
        })
        .flatten()
        .zip(2..)
}

fn part1(input: String) -> String {
    input_to_stream(&input)
        .filter_map(|(reg, cycle)| {
            if (cycle + 20) % 40 == 0 {
                Some(reg * cycle)
            } else {
                None
            }
        })
        .sum::<i32>()
        .to_string()
}

fn part2(input: String) -> String {
    print!("#");
    for (reg, cycle) in input_to_stream(&input) {
        let cycle = (cycle - 1) % 40 + 1;
        let vis = cycle >= reg && cycle < reg + 3;
        print!("{}", if vis { "#" } else { "." });
        if cycle % 40 == 0 {
            println!();
        }
    }
    println!();
    String::new()
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
