use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
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
    let result = part2(input);

    println!("Result: {}", result);
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();

    let directions = lines.next().expect("First line should be directions");

    lines.next(); // Skip blank line

    let mut nodes = HashMap::new();

    for line in lines {
        let node = parse_node(line);
        nodes.insert(node.id, node);
    }

    let starting_nodes = nodes
        .values()
        .filter(|n| n.id.ends_with("A"))
        .map(|n| n.id)
        .collect::<Vec<&str>>();

    let looping_sequences = starting_nodes
        .iter()
        .map(|n| create_sequence(n, directions, nodes.clone()))
        .collect::<Vec<LoopingSequence>>();

    let mut valid_sets = looping_sequences
        .iter()
        .map(|_| HashSet::new())
        .collect::<Vec<HashSet<u64>>>();

    let mut loop_index = 0;
    loop {
        for (sequence_index, looping_sequence) in looping_sequences.iter().enumerate() {
            let valid_indexes = looping_sequence.valid_after_loops(loop_index as u64);

            let valid_set = &mut valid_sets[sequence_index];

            valid_indexes.iter().for_each(|i| {
                valid_set.insert(*i);
            });
        }

        let mut iter = valid_sets.iter();
        let mut intersection_set = match iter.next() {
            Some(set) => set.clone(),
            None => HashSet::new(),
        };

        for set in iter {
            intersection_set = intersection_set.intersection(set).cloned().collect();
        }

        if !intersection_set.is_empty() {
            // Return minimum value from set
            return *intersection_set.iter().min().unwrap() as u64;
        }

        loop_index += 1;
    }
}

#[derive(Debug)]
struct LoopingSequence {
    total_length: u64,
    loop_start: u64,
    finish_indexes: Vec<u64>,
}

impl LoopingSequence {
    fn loop_length(&self) -> u64 {
        self.total_length - self.loop_start
    }

    fn valid_after_loops(&self, loops: u64) -> Vec<u64> {
        self.finish_indexes
            .iter()
            .map(|i| i + loops * self.loop_length())
            .collect::<Vec<u64>>()
    }
}

fn create_sequence(start: &str, directions: &str, nodes: HashMap<&str, Node>) -> LoopingSequence {
    let mut node_id = start;
    let mut sequence: Vec<(usize, &str)> = vec![(0, node_id)];

    for _ in 0..100 {
        for (i, direction) in directions.chars().enumerate() {
            let current_node = nodes.get(node_id).expect("Invalid node");
            node_id = current_node.step(direction);

            let existing = sequence.iter().find(|(j, id)| i == *j && node_id == *id);

            if existing.is_some() {
                return LoopingSequence {
                    total_length: sequence.len() as u64,
                    loop_start: i as u64 + 1,
                    finish_indexes: sequence
                        .iter()
                        .enumerate()
                        .filter_map(|(i, (_, id))| {
                            if id.ends_with("Z") {
                                Some(i as u64)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<u64>>(),
                };
            } else {
                sequence.push((i, node_id));
            }
        }
    }
    panic!("COULNDT FIND A LOOPING SEQUENCE");
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
    fn test_works() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = part2(input);
        assert_eq!(result, 6);
    }
}

// 11A - L - 11B
// 11B - R -
