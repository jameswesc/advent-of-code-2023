/**
 * Normally I would use a regex to solve this. However,
 * to better learn I'll solve without.
 *
 * The steps are:
 * 1. Read the input file
 * 2. Loop through each line
 * 3. Loop through each character of the line
 * 4. Try to parse a first and second digit. Return first * 10 + second, else return 0.
 * 5. Sum the result of each line
 */

fn main() {
    let input: &str = include_str!("./puzzle-input-1.txt");
    let result = part1(input);

    println!("Result: {}", result);
}

fn parse_line(line: &str) -> i32 {
    let mut first_digit: Option<i32> = None;
    let mut second_digit: Option<i32> = None;

    // loop through each character of this line
    for c in line.chars() {
        // if the character is a number, parse it
        if c.is_digit(10) {
            let num: i32 = c.to_digit(10).unwrap() as i32;

            // if we haven't set the first digit, set it
            if first_digit.is_none() {
                first_digit = Some(num);
                second_digit = Some(num);
            } else {
                // otherwise set the second digit
                second_digit = Some(num);
            }
        }
    }

    if first_digit.is_some() && second_digit.is_some() {
        return first_digit.unwrap() * 10 + second_digit.unwrap();
    }

    return 0;
}

fn part1(input: &str) -> i32 {
    let sum: i32 = input.lines().map(parse_line).sum();
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_works_for_two_numbers() {
        let result = parse_line("ab1-2c");
        assert_eq!(result, 12);
    }

    #[test]
    fn parse_line_works_for_one_number() {
        let result = parse_line("ab1c");
        assert_eq!(result, 11);
    }

    #[test]
    fn parse_line_works_for_no_numbers() {
        let result = parse_line("abc");
        assert_eq!(result, 0)
    }

    #[test]
    fn parse_line_works_for_three_numbers() {
        let result = parse_line("ab1-2-3c");
        assert_eq!(result, 13)
    }

    #[test]
    fn part1_works() {
        let input: &str = include_str!("./test-input-1.txt");

        for line in input.lines() {
            let line_result = parse_line(line);
            // This is cool! It only prints out if a test fails
            println!("Line: {}, Result: {}", line, line_result);
        }

        let result = part1(input);

        assert_eq!(result, 142);
    }
}
