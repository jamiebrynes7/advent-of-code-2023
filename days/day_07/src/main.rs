use std::{cmp::Ordering, collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("../input.txt");
    let hands: Vec<Hand> = utils::parse_lines(input).unwrap();
    get_result::<JokerlessRuleset>(&hands);
    get_result::<JokerRuleset>(&hands);
}

fn get_result<T: Ruleset>(hands: &Vec<Hand>) {
    let mut hands = hands.clone();
    sort::<T>(&mut hands);

    let result = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid as usize)
        .sum::<usize>();

    println!("Result: {result}");
}

trait Ruleset {
    const ORDERING: [char; 13];

    fn evaluate(hand: &Hand) -> HandKind;
}

fn sort<T: Ruleset>(cards: &mut Vec<Hand>) {
    cards.sort_by(|first, second| {
        let first_hand = T::evaluate(first);
        let second_hand = T::evaluate(second);

        let hand_comparison = first_hand.cmp(&second_hand);

        if let Ordering::Equal = hand_comparison {
            let zipped = first.cards.iter().zip(second.cards.iter());
            for (own_char, other_char) in zipped {
                let own_idx = T::ORDERING.iter().position(|c| c == own_char).unwrap();
                let other_idx = T::ORDERING.iter().position(|c| c == other_char).unwrap();

                match own_idx.cmp(&other_idx) {
                    Ordering::Equal => continue,
                    Ordering::Less => return Ordering::Greater,
                    Ordering::Greater => return Ordering::Less,
                }
            }
        }

        hand_comparison
    });
}

struct JokerlessRuleset;

impl Ruleset for JokerlessRuleset {
    const ORDERING: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    fn evaluate(hand: &Hand) -> HandKind {
        let mut card_counts = HashMap::new();

        for c in &hand.cards {
            let count = card_counts.entry(c).or_insert(0u32);
            *count += 1;
        }

        match card_counts.len() {
            5 => HandKind::HighCard,
            4 => HandKind::OnePair,
            3 => {
                // Its either two pair or three of a kind
                let max = *card_counts.values().max().unwrap();
                if max == 3 {
                    HandKind::ThreeOfAKind
                } else {
                    HandKind::TwoPair
                }
            }
            2 => {
                // Its either full house or 4 of a kind
                let max = *card_counts.values().max().unwrap();
                if max == 4 {
                    HandKind::FourOfAKind
                } else {
                    HandKind::FullHouse
                }
            }
            1 => HandKind::FiveOfAKind,
            _ => panic!("Found unexpected number of unique cards"),
        }
    }
}

struct JokerRuleset;

impl Ruleset for JokerRuleset {
    const ORDERING: [char; 13] = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    fn evaluate(hand: &Hand) -> HandKind {
        let mut card_counts = HashMap::new();
        let mut non_joker_count = 0;

        for c in &hand.cards {
            if *c == 'J' {
                continue;
            }

            non_joker_count += 1;
            let count = card_counts.entry(c).or_insert(0u32);
            *count += 1;
        }

        match non_joker_count {
            5 => JokerlessRuleset::evaluate(hand),
            4 => match card_counts.len() {
                1 => HandKind::FiveOfAKind,
                2 => {
                    // Currently could have 2 pairs or 3 of a kind
                    let max = *card_counts.values().max().unwrap();
                    if max == 3 {
                        HandKind::FourOfAKind
                    } else {
                        HandKind::FullHouse
                    }
                }
                3 => HandKind::ThreeOfAKind,
                4 => HandKind::OnePair,
                _ => panic!("Unexpected number of unique cards"),
            },
            3 => match card_counts.len() {
                1 => HandKind::FiveOfAKind,
                2 => HandKind::FourOfAKind,
                3 => HandKind::ThreeOfAKind,
                _ => panic!("Unexpected number of unique cards"),
            },
            2 => {
                // If we have a pair -> can still make 5 of a kind
                if card_counts.len() == 1 {
                    HandKind::FiveOfAKind
                } else {
                    HandKind::FourOfAKind
                }
            }
            1 => HandKind::FiveOfAKind,
            0 => HandKind::FiveOfAKind,
            _ => panic!("Unexpected non-joker count"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");

        let cards = parts.next().unwrap().chars().collect();
        let bid = parts.next().unwrap().parse::<u32>().unwrap();

        Ok(Hand { cards, bid })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
