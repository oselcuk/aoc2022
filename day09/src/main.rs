use std::{collections::HashSet, env, fs};

fn _pprint(h: (i32, i32), t: (i32, i32), seen: &HashSet<(i32, i32)>) {
    for i in 0..5 {
        for j in 0..5 {
            print!(
                "{}",
                match (i, j) {
                    p if p == h => "H",
                    p if p == t => "T",
                    p if seen.contains(&p) => "#",
                    _ => ".",
                }
            )
        }
        println!();
    }
    println!("-----");
}

fn part1(input: String) -> String {
    let mut h = (0, 0);
    let mut t = (0, 0);
    let mut seen = HashSet::<(i32, i32)>::new();
    seen.insert(t);
    // pprint(h, t, &seen);
    for line in input.lines() {
        let (dir, count) = line.split_once(" ").unwrap();
        let count: i32 = count.parse().unwrap();
        for _ in 0..count {
            match dir {
                "R" => h.0 += 1,
                "L" => h.0 -= 1,
                "U" => h.1 += 1,
                "D" => h.1 -= 1,
                _ => panic!(),
            }
            let dx = h.0 - t.0;
            let dy = h.1 - t.1;
            if dx == 0 && dy.abs() > 1 {
                t.1 += dy.signum();
            } else if dy == 0 && dx.abs() > 1 {
                t.0 += dx.signum();
            } else if dx.abs() > 1 || dy.abs() > 1 {
                t.0 += dx.signum();
                t.1 += dy.signum();
            }
            seen.insert(t);
            // pprint(h, t, &seen);
        }
    }

    seen.len().to_string()
}

fn part2(input: String) -> String {
    let n = 10;
    let mut knots = vec![(0, 0); n];
    let mut seen = HashSet::<(i32, i32)>::new();
    seen.insert(*knots.last().unwrap());
    // pprint(h, t, &seen);
    fn move_knot(h: (i32, i32), mut t: (i32, i32)) -> (i32, i32) {
        let dx = h.0 - t.0;
        let dy = h.1 - t.1;
        if dx == 0 && dy.abs() > 1 {
            t.1 += dy.signum();
        } else if dy == 0 && dx.abs() > 1 {
            t.0 += dx.signum();
        } else if dx.abs() > 1 || dy.abs() > 1 {
            t.0 += dx.signum();
            t.1 += dy.signum();
        }
        t
    }
    for line in input.lines() {
        let (dir, count) = line.split_once(" ").unwrap();
        let count: i32 = count.parse().unwrap();
        for _ in 0..count {
            match dir {
                "R" => knots[0].0 += 1,
                "L" => knots[0].0 -= 1,
                "U" => knots[0].1 += 1,
                "D" => knots[0].1 -= 1,
                _ => panic!(),
            }
            for i in 0..n - 1 {
                knots[i + 1] = move_knot(knots[i], knots[i + 1]);
            }
            seen.insert(*knots.last().unwrap());
            // pprint(h, t, &seen);
        }
    }

    seen.len().to_string()
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
