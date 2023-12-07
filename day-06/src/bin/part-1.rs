use regex::Regex;

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    // speeds / holds are equivalent
    fn winning_holds(&self) -> Vec<u64> {
        let mut holds = vec![];
        for hold in 1..self.time {
            let distance = distance_travelled(hold, self.time - hold);
            if distance > self.record_distance {
                holds.push(hold);
            }
        }
        holds
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> u64 {
    let races = parse_input(input);

    races
        .iter()
        .map(|race| race.winning_holds().len() as u64)
        .fold(1, |acc, x| acc * x)
}

// In this example 1 ms of hold 1 mm/ms
// fn hold_to_speed(hold_time: u64) -> u64 {
//     // technically = 1 * hold_time + 0
//     hold_time
// }

fn distance_travelled(speed: u64, time: u64) -> u64 {
    speed * time
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let numbers_regex = Regex::new(r"(\d+)").unwrap();

    let times = lines
        .next()
        .expect("Should be a times line")
        .split(":")
        .nth(1)
        .expect("Should be a second item");

    let times: Vec<u64> = numbers_regex
        .captures_iter(times)
        .map(|cap| cap[1].parse::<u64>().expect("Time should be an integer"))
        .collect();

    let distances = lines
        .next()
        .expect("Should be a times line")
        .split(":")
        .nth(1)
        .expect("Should be a second item");

    let distances: Vec<u64> = numbers_regex
        .captures_iter(distances)
        .map(|cap| cap[1].parse::<u64>().expect("Time should be an integer"))
        .collect();

    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, record_distance)| Race {
            time,
            record_distance,
        })
        .collect::<Vec<Race>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winning_holds() {
        let race = Race {
            time: 7,
            record_distance: 9,
        };
        assert_eq!(race.winning_holds(), vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_distance_travelled() {
        assert_eq!(distance_travelled(1, 6), 6);
        assert_eq!(distance_travelled(5, 2), 10);
    }

    #[test]
    fn test_parse_input() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = parse_input(input);
        assert_eq!(
            result,
            vec![
                Race {
                    time: 7,
                    record_distance: 9
                },
                Race {
                    time: 15,
                    record_distance: 40
                },
                Race {
                    time: 30,
                    record_distance: 200
                }
            ]
        );
    }

    #[test]
    fn it_works() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = part1(input);

        assert_eq!(result, 288);
    }
}
