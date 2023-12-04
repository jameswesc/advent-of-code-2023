use regex::Regex;

fn main() {
    // Your code here
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("Result: {}", result);
}

fn is_egnine_number_adjacent(
    gear: (usize, usize),
    engine_number: (i32, usize, usize, usize),
) -> bool {
    let (gear_row, gear_col) = gear;
    let (_part_number, engine_row, engine_col_start, engine_col_end) = engine_number;

    let engine_row_min = if engine_row > 0 {
        engine_row - 1
    } else {
        engine_row
    };
    let engine_col_min = if engine_col_start > 0 {
        engine_col_start - 1
    } else {
        engine_col_start
    };

    if gear_row >= engine_row_min && gear_row <= engine_row + 1 {
        if gear_col >= engine_col_min && gear_col <= engine_col_end {
            return true;
        }
    }

    return false;
}

fn part2(input: &str) -> i32 {
    // A hash map for all our symbols, indexed by row, col
    let mut gears: Vec<(usize, usize)> = Vec::new();

    // And engine number is tuple with the following format
    // (part_number, row, col_start, col_end)
    let mut engine_numbers: Vec<(i32, usize, usize, usize)> = Vec::new();

    input.lines().enumerate().for_each(|(row, line)| {
        // Regex to capture either a number, or anything that is not a number or .
        let re = Regex::new(r"(\d+)|(\*)").unwrap();

        re.captures_iter(line).for_each(|cap| {
            let digit_match = cap.get(1);
            if digit_match.is_some() {
                let digit_match = digit_match.unwrap();
                let part_number = digit_match.as_str().parse::<i32>().unwrap();
                engine_numbers.push((part_number, row, digit_match.start(), digit_match.end()));
            }

            let gear_match = cap.get(2);
            if gear_match.is_some() {
                let gear_match = gear_match.unwrap();
                gears.push((row, gear_match.start()));
            }
        });
    });

    // println!("GEARS\n{:?}\n", gears);
    // println!("ENGINE NUMBERS\n{:?}\n", engine_numbers);

    let result = gears
        .iter()
        .filter_map(|gear| {
            let adjacent_engine_numbers: Vec<i32> = engine_numbers
                .iter()
                .filter_map(|engine_number| {
                    if is_egnine_number_adjacent(*gear, *engine_number) {
                        Some(engine_number.0)
                    } else {
                        None
                    }
                })
                .collect();

            if adjacent_engine_numbers.len() == 2 {
                return Some(adjacent_engine_numbers[0] * adjacent_engine_numbers[1]);
            } else {
                return None;
            }
        })
        .sum();

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Your test code here
        let input = include_str!("./test-input.txt");
        let result = part2(input);
        assert_eq!(result, 467835);
    }
}
