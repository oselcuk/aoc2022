use std::{env, fs, ops::RangeInclusive};

fn _pprint(orig: &Vec<Vec<bool>>, map: &Vec<Vec<bool>>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            print!(
                "{}",
                if orig[i][j] {
                    '#'
                } else if map[i][j] {
                    'o'
                } else {
                    '.'
                }
            )
        }
        println!();
    }
}

fn parse_input(input: String) -> (Vec<Vec<bool>>, usize) {
    let rocks = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|r| -> (usize, usize) {
                    let (x, y) = r.split_once(',').unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<Vec<(usize, usize)>>>();
    let y_max: usize = rocks.iter().flatten().map(|(_, y)| *y).max().unwrap() + 1;
    let x_min = 500 - y_max - 1;
    let x_max = 500 + y_max + 1;

    let mut map = vec![vec![false; x_max - x_min + 1]; y_max + 1];
    fn range(l: usize, r: usize) -> RangeInclusive<usize> {
        if l < r {
            l..=r
        } else {
            r..=l
        }
    }
    for rock in rocks {
        for window in rock.windows(2) {
            if let [(x1, y1), (x2, y2)] = window {
                for x in range(*x1, *x2) {
                    for y in range(*y1, *y2) {
                        map[y][x - x_min] = true
                    }
                }
            }
        }
    }
    (map, 500 - x_min)
}

fn simulate(map: &mut Vec<Vec<bool>>, sand: usize) -> i32 {
    let d = map.len();
    let w = map[0].len();
    for i in 0.. {
        let mut x = sand;
        let mut y = 0;
        loop {
            if y + 1 == d {
                return i;
            }
            y += 1;
            if !map[y][x] {
                continue;
            }
            if x == 0 {
                return i;
            }
            if !map[y][x - 1] {
                x -= 1;
                continue;
            }
            if x + 1 == w {
                return i;
            }
            if !map[y][x + 1] {
                x += 1;
                continue;
            }
            y -= 1;
            break;
        }
        if map[y][x] {
            return i;
        }
        map[y][x] = true;
    }
    0
}

fn part1(input: String) -> String {
    let (orig, sand) = parse_input(input);
    let mut map = orig.clone();
    let result = simulate(&mut map, sand);
    // _pprint(&orig, &map);
    result.to_string()
}

fn part2(input: String) -> String {
    let (mut orig, sand) = parse_input(input);
    let w = orig[0].len();
    orig.push(vec![true; w]);
    let mut map = orig.clone();
    let result = simulate(&mut map, sand);
    // _pprint(&orig, &map);
    result.to_string()
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
