use std::fmt;
use std::str::Lines;

// -------- SEED INTERVALS ---------

#[derive(Debug)]
struct SeedInterval {
    start: u64,
    range: u64,
}

impl PartialEq for SeedInterval {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.range == other.range
    }
}

impl fmt::Display for SeedInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.start, self.end())
    }
}

impl SeedInterval {
    fn end(&self) -> u64 {
        self.start + self.range - 1
    }
}

// -------- TRANSFORMS ---------

#[derive(Debug)]
struct Transform {
    dest_start: u64,
    source_start: u64,
    range: u64,
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({},{}) -> ({},{})",
            self.source_start,
            self.end(),
            self.dest_start,
            self.dest_end()
        )
    }
}

impl PartialEq for Transform {
    fn eq(&self, other: &Self) -> bool {
        self.dest_start == other.dest_start
            && self.source_start == other.source_start
            && self.range == other.range
    }
}

impl Transform {
    fn transform_value(&self, value: u64) -> u64 {
        if value < self.source_start || value > self.source_start + self.range {
            return value;
        }

        self.dest_start + (value - self.source_start)
    }

    fn end(&self) -> u64 {
        self.source_start + self.range - 1
    }

    fn dest_end(&self) -> u64 {
        self.dest_start + self.range - 1
    }
}

// -------- Mappings ---------

#[derive(Debug)]
struct Mapping {
    transforms: Vec<Transform>,
}

impl PartialEq for Mapping {
    fn eq(&self, other: &Self) -> bool {
        self.transforms == other.transforms
    }
}

impl Mapping {
    fn transform_seed_intervals(&self, seed_intervals: Vec<SeedInterval>) -> Vec<SeedInterval> {
        return vec![];
    }
}

// --------- Main ----------

fn transform_seed_interval(seed: SeedInterval, transform: Transform) -> Vec<SeedInterval> {
    // One seed interval can be transformed into at most 3 seed intervals
    // one before the transform range, one inside the transform range, and one after the transform range

    // There are 6 cases
    // 1. Seed interval is before the transform range
    // 2. Seed interval is inside the transform range
    // 3. Seed interval is after the transform range
    // 4. Seed interval starts before the transform range and ends inside the transform range
    // 5. Seed interval starts inside the transform range and ends after the transform range
    // 6. Seed interval starts before the transform range and ends after the transform range

    if seed.end() < transform.source_start {
        // Case 1
        return vec![SeedInterval {
            start: seed.start,
            range: seed.range,
        }];
    } else if seed.start >= transform.source_start && seed.end() <= transform.end() {
        // Case 2
        return vec![SeedInterval {
            start: transform.transform_value(seed.start),
            range: seed.range,
        }];
    } else if seed.start > transform.end() {
        // Case 3
        return vec![SeedInterval {
            start: seed.start,
            range: seed.range,
        }];
    } else if seed.start < transform.source_start
        && seed.end() >= transform.source_start
        && seed.end() <= transform.end()
    {
        // Case 4
        let before_transform = SeedInterval {
            start: seed.start,
            range: transform.source_start - seed.start,
        };
        let inside_transform = SeedInterval {
            start: transform.transform_value(transform.source_start),
            range: seed.end() - transform.source_start + 1,
        };
        return vec![before_transform, inside_transform];
    } else if seed.start >= transform.source_start
        && seed.start <= transform.end()
        && seed.end() > transform.end()
    {
        // Case 5
        let inside_range = transform.end() - seed.start + 1;
        let inside_transform = SeedInterval {
            start: transform.transform_value(seed.start),
            range: inside_range,
        };
        let after_transform = SeedInterval {
            start: seed.start + inside_range,
            range: seed.range - inside_range,
        };
        return vec![inside_transform, after_transform];
    } else if seed.start < transform.source_start && seed.end() > transform.end() {
        // Case 6
        let before_range = transform.source_start - seed.start;
        let before_transform = SeedInterval {
            start: seed.start,
            range: before_range,
        };
        let inside_transform = SeedInterval {
            start: transform.transform_value(transform.source_start),
            range: transform.range,
        };
        let after_transform = SeedInterval {
            start: seed.start + before_range + transform.range,
            range: seed.range - transform.range - before_range,
        };
        return vec![before_transform, inside_transform, after_transform];
    }

    panic!("All cases should be captured");
}

fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines(); // create iterator

    let mut seed_intervals = parse_seeds(&mut lines); // parse seeds

    let mut mappings = Vec::new();

    lines.next(); // discard blank line
    lines.next(); // discard seed-to-soil map line
    mappings.push(parse_mapping(&mut lines));

    lines.next(); // discared soil-to-fertilizer map line
    mappings.push(parse_mapping(&mut lines));

    lines.next(); // discard fertilizer-to-water map line
    mappings.push(parse_mapping(&mut lines));

    lines.next(); // discard water-to-light map line
    mappings.push(parse_mapping(&mut lines));

    lines.next(); // discard light-to-temperature map line
    mappings.push(parse_mapping(&mut lines));

    lines.next(); // discard temperature-to-humidity map line
    mappings.push(parse_mapping(&mut lines));

    lines.next(); // discard humidity-to-location map line
    mappings.push(parse_mapping(&mut lines));

    // mappings.into_iter().for_each(|mapping| {
    //     println!("{:?}", mapping);
    // });

    let seed_to_soil = mappings.get(0).expect("Should be a mappings");

    let soil_intervals = seed_to_soil.transform_seed_intervals(seed_intervals);

    soil_intervals
        .iter()
        .for_each(|soil_interval| println!("{}", soil_interval));

    0
}

fn parse_seeds(lines: &mut Lines) -> Vec<SeedInterval> {
    let numbers = lines
        .next()
        .expect("Should be a first line")
        .get(7..)
        .expect("Seeds should start from 7th char")
        .split(" ")
        .map(|s| s.parse::<u64>().expect("Should be a number"))
        .collect::<Vec<u64>>();

    let mut intervals = Vec::new();

    for i in (0..numbers.len()).step_by(2) {
        let start = numbers[i];
        let range = numbers[i + 1];
        intervals.push(SeedInterval { start, range })
    }

    intervals
}

fn parse_mapping(lines: &mut Lines) -> Mapping {
    let mut transforms = Vec::new();

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

        transforms.push(Transform {
            dest_start,
            source_start,
            range,
        })
    }

    Mapping { transforms }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_seeds_works() {
        let input = "seeds: 79 14 55 13";

        let result = parse_seeds(&mut input.lines());

        assert_eq!(
            result,
            vec![
                SeedInterval {
                    start: 79,
                    range: 14
                },
                SeedInterval {
                    start: 55,
                    range: 13
                }
            ]
        );
    }

    #[test]
    fn parse_mappings_works() {
        let input = "50 98 2
        52 50 48";
        let mut lines = input.lines();

        let result = parse_mapping(&mut lines);

        assert_eq!(
            result,
            Mapping {
                transforms: vec![
                    Transform {
                        dest_start: 50,
                        source_start: 98,
                        range: 2
                    },
                    Transform {
                        dest_start: 52,
                        source_start: 50,
                        range: 48
                    }
                ]
            }
        );
    }

    #[test]
    fn transform_seed_interval_case_1() {
        let seed_interval = SeedInterval {
            start: 0,
            range: 10,
        };
        let transform = Transform {
            dest_start: 100,
            source_start: 200,
            range: 10,
        };

        let result = transform_seed_interval(seed_interval, transform);

        assert_eq!(
            result,
            vec![SeedInterval {
                start: 0,
                range: 10
            }]
        );
    }

    #[test]
    fn transform_seed_interval_case_2() {
        let seed_interval = SeedInterval {
            start: 200,
            range: 10,
        };
        let transform = Transform {
            dest_start: 100,
            source_start: 200,
            range: 10,
        };

        let result = transform_seed_interval(seed_interval, transform);

        assert_eq!(
            result,
            vec![SeedInterval {
                start: 100,
                range: 10
            }]
        );
    }

    #[test]
    fn transform_seed_interval_case_3() {
        let seed_interval = SeedInterval {
            start: 210,
            range: 10,
        };
        let transform = Transform {
            dest_start: 100,
            source_start: 200,
            range: 10,
        };

        let result = transform_seed_interval(seed_interval, transform);

        assert_eq!(
            result,
            vec![SeedInterval {
                start: 210,
                range: 10
            }]
        );
    }

    #[test]
    fn transform_seed_interval_case_4() {
        let seed_interval = SeedInterval {
            start: 190,
            range: 20,
        };
        let transform = Transform {
            dest_start: 100,
            source_start: 200,
            range: 10,
        };

        let result = transform_seed_interval(seed_interval, transform);

        assert_eq!(
            result,
            vec![
                SeedInterval {
                    start: 190,
                    range: 10
                },
                SeedInterval {
                    start: 100,
                    range: 10
                }
            ]
        );
    }

    #[test]
    fn transform_seed_interval_case_5() {
        let seed_interval = SeedInterval {
            start: 205,
            range: 20,
        };
        let transform = Transform {
            dest_start: 100,
            source_start: 200,
            range: 10,
        };

        let result = transform_seed_interval(seed_interval, transform);

        assert_eq!(
            result,
            vec![
                SeedInterval {
                    start: 105,
                    range: 5
                },
                SeedInterval {
                    start: 210,
                    range: 15
                }
            ]
        );
    }

    #[test]
    fn transform_seed_interval_case_6() {
        let seed_interval = SeedInterval {
            start: 190,
            range: 30,
        };
        let transform = Transform {
            dest_start: 100,
            source_start: 200,
            range: 10,
        };

        let result = transform_seed_interval(seed_interval, transform);

        assert_eq!(
            result,
            vec![
                SeedInterval {
                    start: 190,
                    range: 10
                },
                SeedInterval {
                    start: 100,
                    range: 10
                },
                SeedInterval {
                    start: 210,
                    range: 10
                }
            ]
        );
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

        let result = part2(input);

        assert_eq!(result, 46);
    }
}
