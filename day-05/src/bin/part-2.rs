use std::fmt;
use std::ops::Range;
use std::str::Lines;

fn map_ranges(inputs: Vec<Range<u64>>, mapping: &Mapping) -> Vec<Range<u64>> {
    let mut outputs = Vec::new();
    let mut inputs = inputs.clone();

    let transforms = &mapping.transforms;

    // println!("Inputs: {:?}", inputs);

    // while pop inputs loop and add to outputs
    while let Some(input) = inputs.pop() {
        // println!("Testing input: {:?}", input);

        let transform = transforms.iter().find_map(|transform| {
            let m = match_ranges(&input, &transform.source);

            match m {
                None => None,
                Some(rm) => Some((rm, transform)),
            }
        });

        if transform.is_none() {
            // println!("No transform found for input: {:?}", input);
            outputs.push(input);
            continue;
        }

        let (range_match, transform) = transform.unwrap();
        // println!(
        //     "Transform: {} found with range match {:?}",
        //     transform, range_match
        // );

        match range_match {
            RangeMatch::Equals => {
                outputs.push(transform.dest.clone());
            }
            RangeMatch::IsInside => {
                let start = transform.map_value(input.start);
                let end = start + (input.end - input.start);
                outputs.push(start..end);
            }
            RangeMatch::IntersectsWith => {
                if input.start < transform.source.start {
                    let outside_range = input.start..transform.source.start;
                    inputs.push(outside_range);

                    let inside_range = transform.source.start..input.end;
                    let start = transform.map_value(inside_range.start);
                    let end = start + (inside_range.end - inside_range.start);
                    outputs.push(start..end);
                } else {
                    let inside_range = input.start..transform.source.end;
                    let start = transform.map_value(inside_range.start);
                    let end = start + (inside_range.end - inside_range.start);
                    outputs.push(start..end);

                    let outside_range = transform.source.end..input.end;
                    inputs.push(outside_range);
                }
            }
            RangeMatch::Spans => {
                outputs.push(transform.dest.clone());

                let before_range = input.start..transform.source.start;
                inputs.push(before_range);

                let after_range = transform.source.end..input.end;
                inputs.push(after_range);
            }
        }
    }

    outputs
}

#[derive(Debug, PartialEq)]
enum RangeMatch {
    Equals,         // A equals B
    IntersectsWith, // A intersects with B
    Spans,          // A spans B
    IsInside,       // A is inside B
}

fn match_ranges(a: &Range<u64>, b: &Range<u64>) -> Option<RangeMatch> {
    if a.start == b.start && a.end == b.end {
        Some(RangeMatch::Equals)
    } else if a.start >= b.start && a.end <= b.end {
        Some(RangeMatch::IsInside)
    } else if a.start <= b.start && a.end >= b.end {
        Some(RangeMatch::Spans)
    } else if (a.start < b.start && a.end > b.start) || (a.start < b.end && a.end > b.end) {
        Some(RangeMatch::IntersectsWith)
    } else {
        None
    }
}

// -------- TRANSFORMS ---------

#[derive(Debug, Clone)]
struct Transform {
    source: Range<u64>,
    dest: Range<u64>,
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({},{}) -> ({},{})",
            self.source.start, self.source.end, self.dest.start, self.dest.end
        )
    }
}

impl Transform {
    fn map_value(&self, value: u64) -> u64 {
        if self.source.contains(&value) {
            let offset = value - self.source.start;
            self.dest.start + offset
        } else {
            value
        }
    }
}

impl PartialEq for Transform {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.dest == other.dest
    }
}

// -------- Mappings ---------

#[derive(Debug, Clone)]
struct Mapping {
    transforms: Vec<Transform>,
}

impl PartialEq for Mapping {
    fn eq(&self, other: &Self) -> bool {
        self.transforms == other.transforms
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines(); // create iterator

    let seeds = parse_seeds(&mut lines); // parse seeds

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

    mappings
        .iter()
        .fold(seeds, |seeds, mapping| map_ranges(seeds, &mapping))
        .iter()
        .map(|range| range.start)
        .min()
        .expect("Should be a minimum")
}

fn parse_seeds(lines: &mut Lines) -> Vec<Range<u64>> {
    let numbers = lines
        .next()
        .expect("Should be a first line")
        .get(7..)
        .expect("Seeds should start from 7th char")
        .split(" ")
        .map(|s| s.parse::<u64>().expect("Should be a number"))
        .collect::<Vec<u64>>();

    let mut seeds = Vec::new();

    for i in (0..numbers.len()).step_by(2) {
        let start = numbers[i];
        let range = numbers[i + 1];
        seeds.push(start..start + range);
    }

    seeds
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
            source: source_start..source_start + range,
            dest: dest_start..dest_start + range,
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

        assert_eq!(result, vec![79..93, 55..68]);
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
                        source: 98..100,
                        dest: 50..52
                    },
                    Transform {
                        source: 50..98,
                        dest: 52..100,
                    }
                ]
            }
        );
    }

    #[test]
    fn map_ranges_works() {
        let inputs = vec![0..100];
        let mapping = Mapping {
            transforms: vec![
                Transform {
                    source: 40..60,
                    dest: 240..260,
                },
                Transform {
                    source: 80..120,
                    dest: 280..320,
                },
            ],
        };

        let result = map_ranges(inputs, &mapping);

        assert_eq!(
            result,
            vec![
                240..260, // from 40..60
                280..300, // from 80..100
                60..80,   // from 60..80
                0..40,    // from 0..40
            ]
        );
    }

    #[test]
    fn map_value_works() {
        let transform = Transform {
            source: 98..100,
            dest: 50..52,
        };

        let result = transform.map_value(99);

        assert_eq!(result, 51);
    }

    #[test]
    fn match_ranges_works() {
        assert_eq!(match_ranges(&(0..10), &(20..30)), None);
        assert_eq!(
            match_ranges(&(0..10), &(5..15)),
            Some(RangeMatch::IntersectsWith)
        );
        assert_eq!(
            match_ranges(&(5..15), &(0..10)),
            Some(RangeMatch::IntersectsWith)
        );
        assert_eq!(match_ranges(&(10..20), &(10..20)), Some(RangeMatch::Equals));
        assert_eq!(
            match_ranges(&(10..20), &(0..50)),
            Some(RangeMatch::IsInside)
        );
        assert_eq!(match_ranges(&(10..50), &(20..30)), Some(RangeMatch::Spans));
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
