use std::{env, fs};

#[derive(Eq, Debug)]
enum Message {
    Num(i32),
    List(Vec<Message>),
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn vec_cmp<'a, Il, Ir>(l: Il, r: Ir, l_len: usize, r_len: usize) -> std::cmp::Ordering
        where
            Il: Iterator<Item = &'a Message>,
            Ir: Iterator<Item = &'a Message>,
        {
            l.zip(r)
                .map(|(l, r)| l.cmp(r))
                .filter(|cmp| !cmp.is_eq())
                .next()
                .unwrap_or(l_len.cmp(&r_len))
        }
        match (self, other) {
            (Message::Num(l), Message::Num(r)) => l.cmp(r),
            (Message::List(l), Message::List(r)) => vec_cmp(l.iter(), r.iter(), l.len(), r.len()),
            (Message::List(l), r @ Message::Num(_)) => {
                vec_cmp(l.iter(), std::iter::once(r), l.len(), 1)
            }
            (l @ Message::Num(_), Message::List(r)) => {
                vec_cmp(std::iter::once(l), r.iter(), 1, r.len())
            }
        }
    }
}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

fn parse_message(msg: &str) -> Message {
    let tokens = msg
        .split(",")
        .flat_map(|s| s.split_inclusive(['[', ']']))
        .flat_map(|s| {
            if s.len() > 1 && s.ends_with(']') {
                vec![&s[..s.len() - 1], &s[s.len() - 1..]]
            } else {
                vec![s]
            }
        });
    let mut stack = vec![vec![]];
    for token in tokens {
        // println!("Token: {token}\nStack: {stack:?}\n");
        match token {
            "[" => stack.push(vec![]),
            "]" => {
                let msg = Message::List(stack.pop().unwrap());
                stack.last_mut().unwrap().push(msg);
            }
            t => stack
                .last_mut()
                .unwrap()
                .push(Message::Num(t.parse().unwrap())),
        }
    }
    stack.pop().unwrap().pop().unwrap()
}

fn part1(input: String) -> String {
    input
        .split("\n\n")
        .map(|s| s.split_once("\n").unwrap())
        .map(|(l, r)| parse_message(l) < parse_message(r))
        .enumerate()
        .filter(|(_, correct)| *correct)
        .map(|(i, _)| i + 1)
        .sum::<usize>()
        .to_string()
}

fn part2(input: String) -> String {
    let mut messages: Vec<Message> = input.split_ascii_whitespace().map(parse_message).collect();
    let d1 = Message::List(vec![Message::List(vec![Message::Num(2)])]);
    let d2 = Message::List(vec![Message::List(vec![Message::Num(6)])]);
    messages.sort();
    let k1 = messages.binary_search(&d1).unwrap_err() + 1;
    let k2 = messages.binary_search(&d2).unwrap_err() + 2;
    (k1 * k2).to_string()
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
