use regex::Regex;
use std::collections::HashMap;

fn main() {
    // Your code here
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("Result: {}", result);
}

fn format_key(col: i32, row: i32) -> String {
    return format!("{},{}", col, row);
}

//

fn generate_adjacent_keys(row: i32, col_start: i32, col_end: i32) -> Vec<String> {
    let mut keys: Vec<String> = Vec::new();

    // If col start is above 0, we can do the column to the left
    let col_start = if col_start > 0 {
        col_start - 1
    } else {
        col_start
    };

    for col in col_start..col_end + 1 {
        // if the row is above 0, we can also do the above row
        if row > 0 {
            keys.push(format_key(col, row - 1));
        }
        keys.push(format_key(col, row));
        keys.push(format_key(col, row + 1));
    }

    return keys;
}

fn part1(input: &str) -> i32 {
    // A hash map for all our symbols, indexed by row, col
    let mut symbols = HashMap::new();

    // And engine number is tuple with the following format
    // (part_number, row, col_start, col_end)
    let mut engine_numbers: Vec<(i32, usize, usize, usize)> = Vec::new();

    input.lines().enumerate().for_each(|(row, line)| {
        // Regex to capture either a number, or anything that is not a number or .
        let re = Regex::new(r"(\d+)|([^\d\.])").unwrap();

        re.captures_iter(line).for_each(|cap| {
            let digit_match = cap.get(1);
            if digit_match.is_some() {
                let digit_match = digit_match.unwrap();
                let part_number = digit_match.as_str().parse::<i32>().unwrap();
                engine_numbers.push((part_number, row, digit_match.start(), digit_match.end()));
            }

            let symbol_match = cap.get(2);
            if symbol_match.is_some() {
                let symbol_match = symbol_match.unwrap();
                let key = format_key(symbol_match.start() as i32, row as i32);
                symbols.insert(key, symbol_match.as_str());
            }
        });
    });

    let mut valid_part_numbers: Vec<i32> = Vec::new();

    engine_numbers
        .iter()
        .for_each(|(part_number, row, col_start, col_end)| {
            let adjacent_keys =
                generate_adjacent_keys(*row as i32, *col_start as i32, *col_end as i32);

            let mut is_adjacent_symbol = false;

            for key in &adjacent_keys {
                if symbols.get(key).is_some() {
                    is_adjacent_symbol = true;
                    // cancel the loop
                    break;
                }
            }

            if is_adjacent_symbol {
                valid_part_numbers.push(*part_number);
            }
        });

    // println!("TEST KEYS\n{:?}\n", generate_adjacent_keys(0, 0, 3));
    // println!("SYMBOLS\n{:?}\n", symbols);
    // println!("ENGINE NUMBERS\n{:?}\n", engine_numbers);
    // println!("VALID PART NUMBERS\n{:?}\n", valid_part_numbers);

    return valid_part_numbers.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Your test code here
        let input = include_str!("./test-input.txt");
        let result = part1(input);
        assert_eq!(result, 4361);
    }
}
