use std::str::Lines;

// GIVING UP ON THIS ONE

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

    fn map_seed_range(&self, seed_range: SeedRange) -> Vec<SeedRange> {
        // before case
        if seed_range.start + seed_range.range - 1 < self.source_start {
            return vec![seed_range];
        // after case
        } else if self.source_start + self.range - 1 < seed_range.start {
            return vec![seed_range];
        // inside case
        } else if seed_range.start >= self.source_start
            && seed_range.start + seed_range.range - 1 <= self.source_start + self.range - 1
        {
            return vec![SeedRange {
                start: self.map_value(seed_range.start).expect("Should be inside"),
                range: seed_range.range,
            }];
        // split start case
        } else if seed_range.start < self.source_start {
            // first seed range will be from start with a range
            // that takes it to the source_start
            // e.g. seed_start = 8, range = 5 -> 8, 9, 10, 11, 12
            // source_start = 10
            // first_seed_range = 2 -> 8, 9
            // second_seed_range = 3 -> 10, 11, 12
            // second_seed_range.start = self.map_value(self.source_start)

            return vec![
                SeedRange {
                    start: seed_range.start,
                    range: self.source_start - seed_range.start,
                },
                SeedRange {
                    start: self.map_value(self.source_start).expect("Should be inside"),
                    range: seed_range.range - (self.source_start - seed_range.start),
                },
            ];
        // split end case
        } else if seed_range.start > self.source_start {
            // self: 10, 11, 12, 13, 14 -> 20, 21, 22, 23, 24
            // seed: 13, 14, 15, 16, 17
            // result should be:
            // 13, 14 -> 23, 24
            // 15, 16, 17 -| 15, 16, 17

            return vec![
                SeedRange {
                    start: self.map_value(seed_range.start).expect("Should be inside"),
                    range: self.source_start + self.range - seed_range.start,
                },
                SeedRange {
                    start: seed_range.start + (self.source_start + self.range - seed_range.start),
                    range: seed_range.range - (self.source_start + self.range - seed_range.start),
                },
            ];
        }

        panic!("Shouldn't get here");
    }

    fn map_seed_ranges(&self, seed_ranges: Vec<SeedRange>) -> Vec<SeedRange> {
        seed_ranges
            .into_iter()
            .flat_map(|seed_range| self.map_seed_range(seed_range))
            .collect()
    }
}

#[derive(Debug)]
struct Mappings {
    ranges: Vec<MappingRange>,
}

impl PartialEq for Mappings {
    fn eq(&self, other: &Self) -> bool {
        self.ranges == other.ranges
    }
}

impl Mappings {
    fn map_seed_ranges(&self, seed_ranges: Vec<SeedRange>) -> Vec<SeedRange> {
        self.ranges
            .iter()
            .fold(seed_ranges, |seed_ranges, mapping_range| {
                mapping_range.map_seed_ranges(seed_ranges)
            })
    }
}

#[derive(Debug)]
struct SeedRange {
    start: u64,
    range: u64,
}

impl PartialEq for SeedRange {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.range == other.range
    }
}

fn main() {
    // Your code here
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> u64 {
    println!("Beginning seed parse");

    // ---- Parse our seeds ----
    let mut lines = input.lines(); // create iterator
    let seeds = parse_seeds(&mut lines); // parse seeds

    // ---- Parse our mappings ----
    let mut mappings = Vec::new();
    lines.next(); // discard blank line
    lines.next(); // discard seed-to-soil map line
    mappings.push(parse_mappings(&mut lines));
    // println!("Parsed seed-to-soil map");

    lines.next(); // discared soil-to-fertilizer map line
    mappings.push(parse_mappings(&mut lines));
    // println!("Parsed soil-to-fertilizer map");

    lines.next(); // discard fertilizer-to-water map line
    mappings.push(parse_mappings(&mut lines));
    // println!("Parsed fertilizer-to-water map");

    lines.next(); // discard water-to-light map line
    mappings.push(parse_mappings(&mut lines));
    // println!("Parsed water-to-light map");

    lines.next(); // discard light-to-temperature map line
    mappings.push(parse_mappings(&mut lines));
    // println!("Parsed light-to-temperature map");

    lines.next(); // discard temperature-to-humidity map line
    mappings.push(parse_mappings(&mut lines));
    // println!("Parsed temperature-to-humidity map");

    lines.next(); // discard humidity-to-location map line
    mappings.push(parse_mappings(&mut lines));
    // println!("Parsed humidity-to-location map");

    // ---- Map our seeds ----
    // TODO

    return 0;
}

// Only update currently is to parse seeds
// This is way too slow though
fn parse_seeds(lines: &mut Lines) -> Vec<SeedRange> {
    let seed_numbers = lines
        .next()
        .expect("Should be a first line")
        .get(7..)
        .expect("Seeds should start from 7th char")
        .split(" ")
        .map(|s| s.parse::<u64>().expect("Seeds should be integers"))
        .collect::<Vec<u64>>();

    let mut seeds = Vec::new();

    for i in 0..seed_numbers.len() / 2 {
        let seed_start = seed_numbers
            .get(i * 2)
            .expect("There should be a seed_start");
        let seed_range = seed_numbers
            .get(i * 2 + 1)
            .expect("There should be a seed_range");

        seeds.push(SeedRange {
            start: *seed_start,
            range: *seed_range,
        });
    }

    return seeds;
}

fn parse_mappings(lines: &mut Lines) -> Mappings {
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

    Mappings { ranges: mappings }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_range_before_mapping_range_works() {
        let mapping_range = MappingRange {
            dest_start: 20,
            source_start: 10,
            range: 5,
        };

        let seed_range = SeedRange { start: 5, range: 5 };
        let result = mapping_range.map_seed_range(seed_range);
        assert_eq!(result, vec![SeedRange { start: 5, range: 5 }]);
    }

    #[test]
    fn seed_range_after_mapping_range_works() {
        let mapping_range = MappingRange {
            dest_start: 20,
            source_start: 10,
            range: 5,
        };

        let seed_range = SeedRange {
            start: 15,
            range: 5,
        };
        let result = mapping_range.map_seed_range(seed_range);
        assert_eq!(
            result,
            vec![SeedRange {
                start: 15,
                range: 5
            }]
        );
    }

    #[test]
    fn seed_range_inside_mapping_range_works() {
        let mapping_range = MappingRange {
            dest_start: 20,
            source_start: 10,
            range: 5,
        };

        let seed_range = SeedRange {
            start: 11,
            range: 3,
        };
        let result = mapping_range.map_seed_range(seed_range);
        assert_eq!(
            result,
            vec![SeedRange {
                start: 21,
                range: 3
            }]
        );
    }

    #[test]
    fn seed_range_split_start_works() {
        let mapping_range = MappingRange {
            dest_start: 20,
            source_start: 10,
            range: 5,
        };
        // mapping: 10, 11, 12, 13, 14 -> 20, 21, 22, 23, 24

        let seed_range = SeedRange { start: 9, range: 3 };
        // seed_range: 9, 10, 11

        // result should be:
        //  9 -| 9
        // 10, 11 -> 20, 21

        let result = mapping_range.map_seed_range(seed_range);
        assert_eq!(
            result,
            vec![
                SeedRange { start: 9, range: 1 },
                SeedRange {
                    start: 20,
                    range: 2
                }
            ]
        );
    }

    #[test]
    fn seed_range_split_end_works() {
        let mapping_range = MappingRange {
            dest_start: 20,
            source_start: 10,
            range: 5,
        }; // input: 10, 11, 12, 13, 14 -> 20, 21, 22, 23, 24

        let seed_range = SeedRange {
            start: 13,
            range: 5,
        }; // 13, 14, 15, 16, 17

        // result should be:
        // 13, 14 -> 23, 24
        // 15, 16, 17 -| 15, 16, 17
        let result = mapping_range.map_seed_range(seed_range);
        assert_eq!(
            result,
            vec![
                SeedRange {
                    start: 23,
                    range: 2
                },
                SeedRange {
                    start: 15,
                    range: 3
                }
            ]
        );
    }

    #[test]
    fn parse_seeds_works() {
        let input = "seeds: 1 2 10 5";
        let mut lines = input.lines();

        let result = parse_seeds(&mut lines);

        assert_eq!(
            result,
            vec![
                SeedRange { start: 1, range: 2 },
                SeedRange {
                    start: 10,
                    range: 5
                }
            ]
        );
    }

    #[test]
    fn test_example_works() {
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

        let result = part2(input);

        assert_eq!(result, 46);
    }
}
