use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("../input.txt");
    let (instructions, nodes) = parse_input(input);

    part1(&instructions, &nodes);
    part2(&instructions, &nodes);
}

const TARGET: &'static str = "ZZZ";

fn part1(instructions: &[Instruction], nodes: &HashMap<String, Node>) {
    let mut current = "AAA";
    let mut inst_iter = instructions.iter().cycle();
    let mut steps = 0;
    while current != TARGET {
        let direction = inst_iter.next().unwrap();
        let node = nodes.get(current).unwrap();

        match direction {
            Instruction::Left => current = &node.left,
            Instruction::Right => current = &node.right,
        }
        steps += 1;
    }

    println!("Part 1 result: {steps}");
}

fn part2(instructions: &[Instruction], nodes: &HashMap<String, Node>) {
    struct Cycle<'a> {
        current: &'a str,
        ending_steps: Option<usize>,
    }

    let mut current = nodes
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| Cycle {
            current: k,
            ending_steps: None,
        })
        .collect::<Vec<Cycle>>();

    let is_completed = |nodes: &Vec<Cycle>| nodes.iter().all(|cycle| cycle.ending_steps.is_some());

    let mut inst_iter = instructions.iter().cycle();
    let mut steps: usize = 0;

    while !is_completed(&current) {
        let direction = inst_iter.next().unwrap();
        steps += 1;

        for cycle in &mut current {
            if cycle.ending_steps.is_some() {
                continue;
            }

            let node = nodes.get(cycle.current).unwrap();
            match direction {
                Instruction::Left => cycle.current = &node.left,
                Instruction::Right => cycle.current = &node.right,
            }

            if cycle.current.ends_with("Z") {
                cycle.ending_steps = Some(steps);
            }
        }
    }

    let result = current
        .iter()
        .map(|cycle| cycle.ending_steps.unwrap())
        .reduce(utils::nums::lcm)
        .unwrap();

    println!("Part 2 result: {result}");
}

fn parse_input(input: &str) -> (Vec<Instruction>, HashMap<String, Node>) {
    let mut lines = input.split("\n");

    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(Instruction::from)
        .collect();

    let nodes: Vec<Node> = utils::parse_lines_iter(lines.skip(1)).unwrap();
    let indexed_nodes = nodes.into_iter().map(|n| (n.ident.clone(), n)).collect();

    (instructions, indexed_nodes)
}

enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'R' => Instruction::Right,
            'L' => Instruction::Left,
            _ => panic!("Unexpected char"),
        }
    }
}

struct Node {
    ident: String,
    left: String,
    right: String,
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Hacky, but oh well.
        let sanitized = s
            .replace(" = ", " ")
            .replace("(", "")
            .replace(")", "")
            .replace(",", "");

        let mut parts = sanitized.split(" ");

        let ident = parts.next().unwrap().to_string();
        let left = parts.next().unwrap().to_string();
        let right = parts.next().unwrap().to_string();

        Ok(Node { ident, left, right })
    }
}
