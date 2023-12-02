use regex::Regex;

fn main() {
    let input = include_str!("./puzzle-input-2.txt");
    let result = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> i32 {
    let sum = input.lines().map(parse_line).sum();
    return sum;
}

/**
 * For this one I want to introduce regex. Otherwise its
 * too annoying...
 *
 * This backfired. match iter with a regex doesnt return overlapping matches.
 * So we have to do things differently.
 * I want to iterate through all the sub strings, then match our regex, but
 * the regex must start at the first character of the string.
 */

fn parse_line(line: &str) -> i32 {
    let re: Regex = Regex::new(r"^(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let digits: Vec<i32> = (0..line.len())
        // get all substrings
        .map(|i| &line[i..])
        // filter out the ones that dont match
        .filter(|s| re.is_match(s))
        // map to digit
        .map(|s| {
            let match_string: &str = re.find(s).unwrap().as_str();
            if match_string.len() == 1 {
                return match_string.parse::<i32>().unwrap();
            } else {
                return spelled_digit_to_int(match_string);
            }
        })
        .collect();

    let first_digit = *digits.first().unwrap();
    let last_digit = *digits.last().unwrap();

    let result = first_digit * 10 + last_digit;
    // println!("Result: {} ({} & {})", result, first_digit, last_digit);

    return result;
}

fn spelled_digit_to_int(digit: &str) -> i32 {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("Unknown digit: {}", digit),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_example_works() {
        let input: &str = "two1nine";
        let result: i32 = parse_line(input);
        assert_eq!(result, 29);

        let input: &str = "eightwothree";
        let result: i32 = parse_line(input);
        assert_eq!(result, 83);

        let input: &str = "abcone2threexyz";
        let result: i32 = parse_line(input);
        assert_eq!(result, 13);

        let input: &str = "xtwone3four";
        let result: i32 = parse_line(input);
        assert_eq!(result, 24);

        let input: &str = "4nineeightseven2";
        let result: i32 = parse_line(input);
        assert_eq!(result, 42);

        let input: &str = "zoneight234";
        let result: i32 = parse_line(input);
        assert_eq!(result, 14);

        let input: &str = "7pqrstsixteen";
        let result: i32 = parse_line(input);
        assert_eq!(result, 76);

        let input: &str = "28gtbkszmrtmnineoneightmx";
        let result: i32 = parse_line(input);
        assert_eq!(result, 28);
    }

    #[test]
    fn part_2_works() {
        let input = include_str!("./test-input-2.txt");
        let result = part2(input);
        assert_eq!(result, 281)
    }
}
