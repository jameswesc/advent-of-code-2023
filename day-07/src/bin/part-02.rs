use std::{cmp::Ordering, collections::HashMap};

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Hash, Copy, Clone)]
enum Card {
    Jack = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut unique_cards: HashMap<Card, u32> = HashMap::new();

        for i in 0..5 {
            let card = self.cards[i];
            if card != Card::Jack {
                let count = unique_cards.get(&card).unwrap_or(&0);
                unique_cards.insert(card, count + 1);
            }
        }

        let num_unique_cards = unique_cards.len();

        match num_unique_cards {
            0 => HandType::FiveOfAKind,
            1 => HandType::FiveOfAKind,
            2 => {
                let mut card_counts = unique_cards.values().collect::<Vec<&u32>>();
                card_counts.sort();

                match card_counts.as_slice() {
                    [1, 1] => HandType::FourOfAKind, // + 3 joker
                    [1, 2] => HandType::FourOfAKind, // + 2 joker
                    [1, 3] => HandType::FourOfAKind, // + 1 joker
                    [2, 2] => HandType::FullHouse,   // + 1 joker
                    [1, 4] => HandType::FourOfAKind,
                    [2, 3] => HandType::FullHouse,
                    _ => panic!("Invalid input"),
                }
            }
            3 => {
                let mut card_counts = unique_cards.values().collect::<Vec<&u32>>();
                card_counts.sort();

                match card_counts.as_slice() {
                    [1, 1, 1] => HandType::ThreeOfAKind, // + 2 jokers
                    [1, 1, 2] => HandType::ThreeOfAKind, // + 1 jokers
                    [1, 1, 3] => HandType::ThreeOfAKind,
                    [1, 2, 2] => HandType::TwoPair,
                    _ => panic!("Invalid input"),
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Card groups should be between 0 and 5"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = self.hand_type().cmp(&other.hand_type());

        if order != Ordering::Equal {
            return order;
        }

        for i in 0..5 {
            let order = self.cards[i].cmp(&other.cards[i]);

            if order != Ordering::Equal {
                return order;
            }
        }

        panic!("Hands should not be equal");
    }
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);

    println!("Part 2: {}", result);
}

fn part2(input: &str) -> u32 {
    let mut hands = input.lines().map(parse_hand).collect::<Vec<Hand>>();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum()
}

fn parse_card(input: char) -> Card {
    match input {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'J' => Card::Jack,
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => panic!("Invalid input"),
    }
}

fn parse_cards(input: &str) -> [Card; 5] {
    let cards: [Card; 5] = input
        .chars()
        .map(parse_card)
        .collect::<Vec<Card>>()
        .try_into()
        .expect("There should be 5 cars");

    cards
}

fn parse_hand(input: &str) -> Hand {
    let mut input = input.split(" ");

    let cards = parse_cards(input.next().expect("There should be cards"));
    let bid = input
        .next()
        .expect("There should be a bid")
        .parse::<u32>()
        .expect("Bid should be a number");

    Hand { cards, bid }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hand_works() {
        let input = "32T3K 765";

        let result = parse_hand(input);

        assert_eq!(
            result,
            Hand {
                cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                bid: 765
            }
        );
    }

    #[test]
    fn ordering_works() {
        let a = parse_hand("33332 1");
        let b = parse_hand("2AAAA 1");
        assert_eq!(a.cmp(&b), Ordering::Greater);
    }

    #[test]
    fn it_works() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = part2(input);

        assert_eq!(result, 5905);
    }
}
