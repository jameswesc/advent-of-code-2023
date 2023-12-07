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

    // There would be a much faster way
    // If you plotted the function of distrance travelled (y)
    // against hold time (x) for a specific max race time
    // you would get some kind of curve
    // The answer you're looking for is where that curve intersects
    // the line y = record_distance

    // But this was quick enough so ¯\_(ツ)_/¯
}

fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> u64 {
    let race = parse_input(input);

    race.winning_holds().len() as u64
}

// In this example 1 ms of hold 1 mm/ms
// fn hold_to_speed(hold_time: u64) -> u64 {
//     // technically = 1 * hold_time + 0
//     hold_time
// }

fn distance_travelled(speed: u64, time: u64) -> u64 {
    speed * time
}

fn parse_input(input: &str) -> Race {
    let mut lines = input.lines();
    let numbers_regex = Regex::new(r"(\d+)").unwrap();

    let times = lines
        .next()
        .expect("Should be a times line")
        .split(":")
        .nth(1)
        .expect("Should be a second item");

    let time = numbers_regex
        .captures_iter(times)
        .fold(String::new(), |acc, cap| acc + &cap[1]);

    let distances = lines
        .next()
        .expect("Should be a times line")
        .split(":")
        .nth(1)
        .expect("Should be a second item");

    let distance = numbers_regex
        .captures_iter(distances)
        .fold(String::new(), |acc, cap| acc + &cap[1]);

    Race {
        time: time.parse::<u64>().expect("Time should be an integer"),
        record_distance: distance
            .parse::<u64>()
            .expect("Distance should be an integer"),
    }
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
            Race {
                time: 71530,
                record_distance: 940200
            }
        );
    }

    #[test]
    fn it_works() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = part2(input);

        assert_eq!(result, 71503);
    }
}
