use std::str::Lines;

#[derive(Debug)]
struct MappingRange {
    dest_start: u64,
    source_start: u64,
    range: u64,
}

impl PartialEq for MappingRange {
    fn eq(&self, other: &Self) -> bool {
        self.dest_start == other.dest_start
            && self.source_start == other.source_start
            && self.range == other.range
    }
}

impl MappingRange {
    fn map_value(&self, value: u64) -> Option<u64> {
        if value < self.source_start || value > self.source_start + self.range {
            return None;
        }

        Some(self.dest_start + (value - self.source_start))
    }
}

#[derive(Debug)]
struct Mapping {
    ranges: Vec<MappingRange>,
}

impl PartialEq for Mapping {
    fn eq(&self, other: &Self) -> bool {
        self.ranges == other.ranges
    }
}

impl Mapping {
    fn map_value(&self, value: u64) -> u64 {
        for range in &self.ranges {
            if let Some(mapped_value) = range.map_value(value) {
                return mapped_value;
            }
        }
        value
    }
}

fn main() {
    // Your code here
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines(); // create iterator

    let seeds = parse_seeds(&mut lines); // parse seeds

    let mut mappings = Vec::new();

    lines.next(); // discard blank line
    lines.next(); // discard seed-to-soil map line
    mappings.push(parse_mappings(&mut lines));

    lines.next(); // discared soil-to-fertilizer map line
    mappings.push(parse_mappings(&mut lines));

    lines.next(); // discard fertilizer-to-water map line
    mappings.push(parse_mappings(&mut lines));

    lines.next(); // discard water-to-light map line
    mappings.push(parse_mappings(&mut lines));

    lines.next(); // discard light-to-temperature map line
    mappings.push(parse_mappings(&mut lines));

    lines.next(); // discard temperature-to-humidity map line
    mappings.push(parse_mappings(&mut lines));

    lines.next(); // discard humidity-to-location map line
    mappings.push(parse_mappings(&mut lines));

    // Return the closest location
    seeds
        .iter()
        .map(|seed| {
            let mut value = *seed;
            for mapping in &mappings {
                value = mapping.map_value(value);
            }
            value
        })
        .min()
        .expect("Should be a min value")
}

fn parse_seeds(lines: &mut Lines) -> Vec<u64> {
    lines
        .next()
        .expect("Should be a first line")
        .get(7..)
        .expect("Seeds should start from 7th char")
        .split(" ")
        .map(|s| s.parse::<u64>().expect("Seeds should be integers"))
        .collect::<Vec<u64>>()
}

fn parse_mappings(lines: &mut Lines) -> Mapping {
    let mut mappings = Vec::new();

    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }

        let mut parts = line.trim().split(" ");
        let dest_start = parts
            .next()
            .expect("Should be a first part")
            .parse::<u64>()
            .expect("Dest should be an integer");
        let source_start = parts
            .next()
            .expect("Should be a second part")
            .parse::<u64>()
            .expect("Source should be an integer");
        let range = parts
            .next()
            .expect("Should be a third part")
            .parse::<u64>()
            .expect("Range should be an integer");

        mappings.push(MappingRange {
            dest_start,
            source_start,
            range,
        })
    }

    Mapping { ranges: mappings }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_seeds_works() {
        let input = "seeds: 79 14 55 13";
        let mut lines = input.lines();

        let result = parse_seeds(&mut lines);

        assert_eq!(result, vec![79, 14, 55, 13]);
    }

    #[test]
    fn parse_mappings_works() {
        let input = "50 98 2
        52 50 48";
        let mut lines = input.lines();

        let result = parse_mappings(&mut lines);

        assert_eq!(
            result,
            Mapping {
                ranges: vec![
                    MappingRange {
                        dest_start: 50,
                        source_start: 98,
                        range: 2
                    },
                    MappingRange {
                        dest_start: 52,
                        source_start: 50,
                        range: 48
                    }
                ]
            }
        );
    }

    #[test]
    fn mapping_range_with_value() {
        let mapping_range = MappingRange {
            dest_start: 50,
            source_start: 98,
            range: 2,
        };

        let result = mapping_range.map_value(99);

        assert_eq!(result, Some(51));
    }

    #[test]
    fn mapping_range_without_value() {
        let mapping_range = MappingRange {
            dest_start: 50,
            source_start: 98,
            range: 2,
        };

        let result = mapping_range.map_value(20);

        assert_eq!(result, None);
    }

    #[test]
    fn mapping_with_value() {
        let mapping = Mapping {
            ranges: vec![
                MappingRange {
                    dest_start: 50,
                    source_start: 98,
                    range: 2,
                },
                MappingRange {
                    dest_start: 52,
                    source_start: 50,
                    range: 48,
                },
            ],
        };

        let result = mapping.map_value(99);

        assert_eq!(result, 51);
    }

    #[test]
    fn mapping_without_value() {
        let mapping = Mapping {
            ranges: vec![
                MappingRange {
                    dest_start: 50,
                    source_start: 98,
                    range: 2,
                },
                MappingRange {
                    dest_start: 52,
                    source_start: 50,
                    range: 48,
                },
            ],
        };

        let result = mapping.map_value(20);

        assert_eq!(result, 20);
    }

    #[test]
    fn it_works() {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";

        let result = part1(input);

        assert_eq!(result, 35);
    }
}
