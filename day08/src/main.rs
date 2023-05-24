use std::{env, fs};

use itertools::iproduct;

fn parse_input(input: String) -> Vec<Vec<i8>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| (c as i8) - ('0' as i8)).collect())
        .collect()
}

fn part1(input: String) -> String {
    let map = parse_input(input);
    let n = map.len();
    let m = map[0].len();
    let mut seen = vec![vec![false; m]; n];
    for i in 0..n {
        let mut max = -1;
        for j in 0..m {
            seen[i][j] = seen[i][j] || (map[i][j] > max);
            max = max.max(map[i][j]);
        }
        max = -1;
        for j in (0..m).rev() {
            seen[i][j] = seen[i][j] || (map[i][j] > max);
            max = max.max(map[i][j]);
        }
    }
    for i in 0..m {
        let mut max = -1;
        for j in 0..n {
            seen[j][i] = seen[j][i] || (map[j][i] > max);
            max = max.max(map[j][i]);
        }
        max = -1;
        for j in (0..n).rev() {
            seen[j][i] = seen[j][i] || (map[j][i] > max);
            max = max.max(map[j][i]);
        }
    }

    seen.iter()
        .map(|row| row.iter().filter(|x| **x).count())
        .sum::<usize>()
        .to_string()
}

fn part2(input: String) -> String {
    let map = parse_input(input);
    let n = map.len();
    let m = map[0].len();
    iproduct!(1..n - 1, 1..m - 1)
        .map(|(x, y)| {
            let mut score = 1;
            let z = map[x][y];
            let mut view = 0;
            // look up
            for i in (0..x).rev() {
                view += 1;
                if map[i][y] >= z {
                    break;
                };
            }
            score *= view;
            view = 0;
            for i in x + 1..n {
                view += 1;
                if map[i][y] >= z {
                    break;
                };
            }
            score *= view;
            view = 0;
            for j in (0..y).rev() {
                view += 1;
                if map[x][j] >= z {
                    break;
                };
            }
            score *= view;
            view = 0;
            for j in y + 1..m {
                view += 1;
                if map[x][j] >= z {
                    break;
                };
            }
            score *= view;
            score
        })
        .max()
        .unwrap()
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
