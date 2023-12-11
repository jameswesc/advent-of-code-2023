use std::collections::HashMap;

#[derive(Debug)]
struct Node<'a> {
    id: &'a str,
    left: &'a str,
    right: &'a str,
}

impl Node<'_> {
    fn step(&self, direction: char) -> &str {
        match direction {
            'L' => self.left,
            'R' => self.right,
            _ => panic!("Invalid direction: {}", direction),
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);

    println!("Result: {}", result);
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();

    let directions = lines.next().expect("First line should be directions");

    lines.next(); // Skip blank line

    let mut nodes = HashMap::new();

    for line in lines {
        let node = parse_node(line);
        nodes.insert(node.id, node);
    }

    let mut current_node = "AAA";
    let mut steps = 0;

    loop {
        for direction in directions.chars() {
            steps += 1;

            let node = nodes.get(current_node).expect("Invalid node");
            current_node = node.step(direction);

            if current_node == "ZZZ" {
                return steps;
            }
        }
    }
}

fn parse_node(line: &str) -> Node {
    let id = &line[0..3];
    let left = &line[7..10];
    let right = &line[12..15];

    Node { id, left, right }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_node_works() {
        let input = "AAA = (BBB, CCC)";
        let result = parse_node(input);
        assert_eq!(result.left, "BBB");
        assert_eq!(result.right, "CCC");
    }

    #[test]
    fn test_1_works() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let result = part1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2_works() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = part1(input);
        assert_eq!(result, 6);
    }
}
