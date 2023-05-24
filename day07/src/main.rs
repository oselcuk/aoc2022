use std::{env, fs};

#[derive(Debug)]
enum Node {
    Dir(Vec<Node>),
    File(usize),
}

impl Node {
    fn size(&self) -> usize {
        match self {
            Node::File(size) => *size,
            Node::Dir(nodes) => nodes.iter().map(Node::size).sum::<usize>(),
        }
    }
}

fn sum_below(node: &Node, limit: usize) -> (usize, usize) {
    match node {
        Node::File(size) => (*size, 0),
        Node::Dir(children) => {
            let (s, l) = children
                .iter()
                .map(|n| sum_below(n, limit))
                .fold((0, 0), |(s1, l1), (s2, l2)| (s1 + s2, l1 + l2));
            (s, if s <= limit { l + s } else { l })
        }
    }
}

fn find_delete(node: &Node, limit: usize) -> (usize, usize) {
    match node {
        Node::File(size) => (*size, usize::MAX),
        Node::Dir(children) => {
            let (s, l) = children
                .iter()
                .map(|n| find_delete(n, limit))
                .fold((0, usize::MAX), |(s1, l1), (s2, l2)| (s1 + s2, l1.min(l2)));
            (s, if s >= limit { l.min(s) } else { l })
        }
    }
}

fn parse_input(lines: Vec<&str>) -> Node {
    let mut stack = Vec::<Node>::new();
    let mut node = Node::Dir(vec![]);

    for line in &lines[1..] {
        let tokens = line.split_ascii_whitespace().collect::<Vec<&str>>();
        match tokens.as_slice() {
            ["$", "cd", ".."] => {
                let top = stack.pop().unwrap();
                if let Node::Dir(mut children) = top {
                    children.push(node);
                    node = Node::Dir(children);
                }
            }
            ["$", "cd", _] => {
                stack.push(node);
                node = Node::Dir(vec![]);
            }
            ["$", "ls"] | ["dir", _] => (),
            [size, _] => {
                if let Node::Dir(mut children) = node {
                    children.push(Node::File(size.parse().unwrap()));
                    node = Node::Dir(children);
                }
            }
            wat => panic!("{wat:?}"),
        }
    }
    while let Some(Node::Dir(mut children)) = stack.pop() {
        children.push(node);
        node = Node::Dir(children);
    }
    node
}

fn part1(input: String) -> String {
    let node = parse_input(input.lines().collect::<Vec<&str>>());
    sum_below(&node, 100_000).1.to_string()
}

fn part2(input: String) -> String {
    let node = parse_input(input.lines().collect::<Vec<&str>>());
    let size = node.size();
    let needed = 30_000_000 + size - 70_000_000;
    find_delete(&node, needed).1.to_string()
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
