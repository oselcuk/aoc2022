use std::{env, fmt::Display, fs};

struct Monkey {
    items: Vec<u64>,
    op: Box<dyn Fn(u64) -> u64>,
    test: u64,
    targets: (usize, usize),
    moves: usize,
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Items: {:?}\n Test: {}\n Targets: {} {}",
            self.items, self.test, self.targets.0, self.targets.1,
        )
    }
}

impl Monkey {
    fn new(inp: &str) -> Self {
        let mut iter = inp.lines();
        iter.next();
        let items = iter
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(2)
            .map(|s| s.trim_end_matches(",").parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let ops = iter
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .collect::<Vec<&str>>();
        let [.., l, op, r] = ops.as_slice() else {panic!()};
        let l: Option<u64> = l.parse().ok();
        let r: Option<u64> = r.parse().ok();
        let op: Box<dyn Fn(u64) -> u64> = match *op {
            "*" => Box::new(move |old| l.unwrap_or(old) * r.unwrap_or(old)),
            "+" => Box::new(move |old| l.unwrap_or(old) + r.unwrap_or(old)),
            _ => panic!("Unexpected op: {}", *op),
        };
        let test = iter
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let targets = iter
            .take(2)
            .map(|l| {
                l.split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            })
            .collect::<Vec<usize>>();
        Self {
            items,
            op,
            test,
            targets: (targets[0], targets[1]),
            moves: 0,
        }
    }
}

fn part1(input: String) -> String {
    let mut monkeys = input
        .split("\n\n")
        .map(Monkey::new)
        .collect::<Vec<Monkey>>();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut items = vec![];
            std::mem::swap(&mut monkeys[i].items, &mut items);
            monkeys[i].moves += items.len();
            for item in items.iter() {
                let m = &monkeys[i];
                let new = (m.op)(*item) / 3;
                let target = if new % m.test == 0 {
                    m.targets.0
                } else {
                    m.targets.1
                };
                monkeys[target].items.push(new);
            }
        }
    }
    monkeys.sort_by_key(|m| -(m.moves as i64));
    (monkeys[0].moves * monkeys[1].moves).to_string()
}

fn part2(input: String) -> String {
    let mut monkeys = input
        .split("\n\n")
        .map(Monkey::new)
        .collect::<Vec<Monkey>>();
    let modulus = monkeys.iter().map(|m| m.test).product::<u64>();
    dbg!(modulus);
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            let mut items = vec![];
            std::mem::swap(&mut monkeys[i].items, &mut items);
            monkeys[i].moves += items.len();
            for item in items.iter() {
                let m = &monkeys[i];
                let new = (m.op)(*item) % modulus;
                let target = if new % m.test == 0 {
                    m.targets.0
                } else {
                    m.targets.1
                };
                monkeys[target].items.push(new);
            }
        }
    }
    monkeys.sort_by_key(|m| -(m.moves as i64));
    (monkeys[0].moves * monkeys[1].moves).to_string()
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
