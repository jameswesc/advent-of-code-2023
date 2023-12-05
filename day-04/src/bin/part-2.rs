use std::collections::HashSet;

fn main() {
    // Your code here
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("Result: {}", result);
}

// We'll want a stack of collected cards. This starts empty.
// We also have a stack of cards. These are processed.
// When a card with index N is processed we:
// 1. Add card N to the stack of collected cars
// 2. Add copies of cards N+1..N+M to the stack of cards, where
// is the number of winning hands in card N (iff M is > 0)

fn part2(input: &str) -> u32 {
    let cards: Vec<(usize, u32)> = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let card_number = index + 1;
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

            (card_number, matching_numbers as u32)
        })
        .collect();

    let mut collected_cards: Vec<(usize, u32)> = Vec::new();

    // Processing order doesn't matter so lets just use Vec
    let mut cards_to_process = cards.clone();

    while let Some(card) = cards_to_process.pop() {
        collected_cards.push(card);

        let winning_numbers = card.1;
        if winning_numbers > 0 {
            // println!("Card {} has {} winning numbers", card.0, winning_numbers);
            let next_card_ndx = card.0;
            let last_card_ndx = card.0 + winning_numbers as usize;
            // println!("Adding cards {}..{}", next_card_ndx, last_card_ndx);

            for ndx in next_card_ndx..last_card_ndx {
                let card_to_add = cards[ndx].clone();
                cards_to_process.push(card_to_add);
            }
        }
    }

    return collected_cards.len() as u32;
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn it_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part2(input);

        assert_eq!(result, 30);
    }
}
