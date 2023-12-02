use regex::Regex;

fn main() {
    let input = include_str!("./input-1.txt");
    let bag = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    let result = part1(input, &bag);
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

fn is_hand_valid(hand: &Cubes, bag: &Cubes) -> bool {
    return hand.red <= bag.red && hand.blue <= bag.blue && hand.green <= bag.green;
}

fn is_game_valid(game: &Game, bag: &Cubes) -> bool {
    for hand in &game.hands {
        if !is_hand_valid(hand, bag) {
            return false;
        }
    }
    return true;
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
fn part1(input: &str, bag: &Cubes) -> i32 {
    let total: i32 = input
        .lines()
        .map(parse_input_line)
        .filter(|game| is_game_valid(game, bag))
        .map(|game| game.id)
        .sum();
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hand_works_single_color() {
        let hand = parse_hand("1 red");
        assert_eq!(hand.red, 1);
        assert_eq!(hand.blue, 0);
        assert_eq!(hand.green, 0);
    }

    #[test]
    fn parse_hand_works_multiple_colors() {
        let hand = parse_hand("1 red, 2 blue, 3 green");
        assert_eq!(hand.red, 1);
        assert_eq!(hand.blue, 2);
        assert_eq!(hand.green, 3);
    }

    #[test]
    fn test_is_valid_hand() {
        let mut hand = Cubes {
            red: 1,
            blue: 1,
            green: 1,
        };
        let bag = Cubes {
            red: 1,
            blue: 1,
            green: 1,
        };
        assert_eq!(is_hand_valid(&hand, &bag), true);

        hand.red = 2;
        assert_eq!(is_hand_valid(&hand, &bag), false);
    }

    #[test]
    fn test_is_game_valid() {
        let bag = Cubes {
            red: 1,
            blue: 1,
            green: 1,
        };
        let game = Game {
            id: 1,
            hands: vec![
                Cubes {
                    red: 2,
                    blue: 1,
                    green: 1,
                },
                Cubes {
                    red: 1,
                    blue: 1,
                    green: 1,
                },
            ],
        };
        assert_eq!(is_game_valid(&game, &bag), false);
    }

    #[test]
    fn test_example() {
        let bag = Cubes {
            red: 12,
            green: 13,
            blue: 14,
        };
        let input = include_str!("./test-input-1.txt");
        let result = part1(input, &bag);
        assert_eq!(result, 8);
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
