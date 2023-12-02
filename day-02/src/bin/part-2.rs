use regex::Regex;

fn main() {
    let input = include_str!("./input-1.txt");
    let result = part2(input);
    println!("Result: {}", result);
}

struct Cubes {
    red: i32,
    blue: i32,
    green: i32,
}

struct Game {
    id: i32,
    hands: Vec<Cubes>,
}

fn calculate_power_set(game: &Game) -> i32 {
    let mut min_bag = Cubes {
        red: 0,
        blue: 0,
        green: 0,
    };

    for hand in &game.hands {
        if hand.red > min_bag.red {
            min_bag.red = hand.red;
        }
        if hand.blue > min_bag.blue {
            min_bag.blue = hand.blue;
        }
        if hand.green > min_bag.green {
            min_bag.green = hand.green;
        }
    }

    return min_bag.red * min_bag.blue * min_bag.green;
}

fn parse_input_line(line: &str) -> Game {
    let re = Regex::new(r"^Game (\d+):(.*)$").unwrap();
    let captures = re.captures(line).unwrap();
    let id: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
    let hand = captures.get(2).unwrap().as_str();
    let hands: Vec<Cubes> = hand.split(";").map(|hand| parse_hand(hand)).collect();

    let game: Game = Game { id, hands };

    return game;
}

fn parse_hand(str: &str) -> Cubes {
    let re = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let mut cubes = Cubes {
        red: 0,
        blue: 0,
        green: 0,
    };

    re.captures_iter(str).for_each(|cap| {
        let count: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let color = cap.get(2).unwrap().as_str();

        match color {
            "red" => cubes.red = count,
            "blue" => cubes.blue = count,
            "green" => cubes.green = count,
            _ => panic!("Unknown color: {}", color),
        }
    });

    return cubes;
}

// TODO - Introduce rules
fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(parse_input_line)
        .map(|game| calculate_power_set(&game))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("./test-input-1.txt");
        let result = part2(input);
        assert_eq!(result, 2286);
    }

    #[test]
    fn test_powerset_1() {
        let result = calculate_power_set(&parse_input_line(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        ));
        assert_eq!(result, 48);
    }

    #[test]
    fn test_powerset_2() {
        let result = calculate_power_set(&parse_input_line(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        ));
        assert_eq!(result, 12);
    }
    #[test]
    fn test_powerset_3() {
        let result = calculate_power_set(&parse_input_line(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        ));
        assert_eq!(result, 1560);
    }
}

impl std::fmt::Display for Cubes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "red: {}, blue: {}, green: {};",
            self.red, self.blue, self.green
        )
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut hands = String::new();
        for hand in &self.hands {
            hands.push_str(&format!("{} ", hand));
        }
        write!(f, "Game {}: {}", self.id, hands)
    }
}
