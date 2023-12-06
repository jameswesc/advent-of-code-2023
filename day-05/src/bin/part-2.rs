use std::fmt;
use std::str::Lines;

#[derive(Debug)]
struct Interval {
    start: u64,
    range: u64,
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.range == other.range
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.start, self.end())
    }
}

impl Interval {
    fn end(&self) -> u64 {
        self.start + self.range - 1
    }

    fn contains_value(&self, value: u64) -> bool {
        value >= self.start && value <= self.end()
    }

    fn is_inside(&self, other: &Interval) -> bool {
        self.start >= other.start && self.end() <= other.end()
    }
    fn is_outside(&self, other: &Interval) -> bool {
        self.end() < other.start || self.start > other.end()
    }
    fn crosses(&self, other: &Interval) -> bool {
        !self.is_inside(other) && !self.is_outside(other)
    }
    fn split_interval(&self, interval_to_split: &Interval) -> Vec<Interval> {
        if interval_to_split.crosses(&self) {
            return vec![Interval {
                start: interval_to_split.start,
                range: interval_to_split.range,
            }];
        } else {
            if interval_to_split.start < &self.start {
                // todo
                // to_split { start: 10, range: 11 } (10, 20)
                // self { start: 15, range: 11 } (15, 25)
                // result: {srart: 10, range: 4 } (10, 14) &
            } else {
                // todo
            }
        }

        panic!("Should have returned split_interval by now")
    }
}

#[derive(Debug)]
struct Transform {
    input: Interval,
    output: Interval,
}
// partial eq
impl PartialEq for Transform {
    fn eq(&self, other: &Self) -> bool {
        self.input == other.input && self.output == other.output
    }
}

impl Transform {
    fn transform_value(&self, value: u64) -> u64 {
        if self.input.contains_value(value) {
            return self.output.start + (value - self.input.start);
        }

        value
    }

    fn transform_interval(&self, interval: &Interval) -> Interval {
        if interval.crosses(&self.input) {
            panic!(
                "Interval {:?} crosses input interval {:?}",
                interval, self.input
            );
        }

        // Transform if inside
        if interval.is_inside(&self.input) {
            return Interval {
                start: self.transform_value(interval.start),
                range: interval.range,
            };
        }

        // Otherwise leave as is
        return Interval {
            start: interval.start,
            range: interval.range,
        };
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

    seeds.into_iter().for_each(|interval| {
        println!("{}", interval);
    });

    0
}

fn parse_seeds(lines: &mut Lines) -> Vec<Interval> {
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
        intervals.push(Interval { start, range })
    }

    intervals
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
                Interval {
                    start: 79,
                    range: 14
                },
                Interval {
                    start: 55,
                    range: 13
                }
            ]
        );
    }

    #[test]
    fn is_inside_works() {
        let a = Interval {
            start: 10,
            range: 10,
        };
        let b = Interval {
            start: 10,
            range: 10,
        };

        assert!(a.is_inside(&b));
    }

    #[test]
    fn is_outside_works() {
        let a = Interval {
            start: 10,
            range: 10,
        };
        let b = Interval {
            start: 0,
            range: 10,
        };
        let c = Interval {
            start: 20,
            range: 10,
        };

        assert!(a.is_outside(&b));
        assert!(a.is_outside(&c));
    }

    #[test]
    fn crosses_works() {
        let a = Interval {
            start: 10,
            range: 10,
        };
        let b = Interval {
            start: 0,
            range: 10,
        };
        let c = Interval {
            start: 5,
            range: 10,
        };
        let d = Interval {
            start: 15,
            range: 10,
        };

        assert!(!a.crosses(&b));
        assert!(a.crosses(&c));
        assert!(a.crosses(&d));
    }

    #[test]
    fn transform_value_works() {
        let transform = Transform {
            input: Interval {
                start: 10,
                range: 10,
            },
            output: Interval {
                start: 50,
                range: 10,
            },
        };

        assert_eq!(transform.transform_value(10), 50);
        assert_eq!(transform.transform_value(15), 55);
        assert_eq!(transform.transform_value(20), 20);
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
