use std::collections::HashSet;

fn main() {
    // Your code here
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> u32 {
    let base: u32 = 2;

    input
        .lines()
        .filter_map(|line| {
            let mut line_split = line.split("|");

            let winning_numbers: HashSet<u32> = line_split
                .next()
                .expect("Should be a left side of |")
                .split(":")
                .nth(1)
                .expect("Should be a right side of :")
                .split(" ")
                .filter_map(|num| num.parse::<u32>().ok())
                .collect();

            let matching_numbers = line_split
                .next()
                .expect("Should be a right side of |")
                .split(" ")
                .filter(|slice| match slice.parse::<u32>() {
                    Ok(n) => winning_numbers.contains(&n),
                    Err(_) => false,
                })
                .count();

            if matching_numbers > 0 {
                Some(base.pow(matching_numbers as u32 - 1))
            } else {
                None
            }
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );

        assert_eq!(result, 13)
    }
}
