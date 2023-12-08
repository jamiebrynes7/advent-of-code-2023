use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use utils::parse_lines;

fn main() {
    let input = include_str!("../input.txt");
    let input: Vec<Card> = parse_lines(input).unwrap();

    part1(&input);
    part2(&input);
}

fn part1(cards: &Vec<Card>) {
    let sum = cards.iter().map(|c| c.value()).sum::<u32>();
    println!("Part 1 result: {sum}");
}

fn part2(cards: &Vec<Card>) {
    // Go in reverse order calculating the total number of scratchcards you'd earn for that
    // scratchcard. Since the cards won are always 'downwards', we only have to calculate
    // each one once.
    let mut cards_per_card: HashMap<u32, u32> = HashMap::new();

    for card in cards.iter().rev() {
        let winning = card.winning_count();
        // Sum up the value of any copied cards
        let copied_count = (0..winning)
            .map(|offset| card.id + 1 + offset as u32)
            .map(|id| *cards_per_card.get(&id).unwrap())
            .sum::<u32>();

        cards_per_card.insert(card.id, copied_count + 1);
    }

    // We have 1 of each card, so lets just sum them up.
    let result = cards_per_card.iter().map(|(_, count)| count).sum::<u32>();

    println!("Part 2 result {result}");
}

struct Card {
    id: u32,
    winning_nums: HashSet<u32>,
    my_nums: HashSet<u32>,
}

impl Card {
    fn winning_count(&self) -> usize {
        self.winning_nums.intersection(&self.my_nums).count()
    }

    fn value(&self) -> u32 {
        let overlap = self.winning_count();

        if overlap == 0 {
            return 0;
        }

        2u32.pow(overlap as u32 - 1)
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(":").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err("Expected 2 parts".into());
        }

        let id_parts = parts[0]
            .split(" ")
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        if id_parts.len() != 2 {
            return Err("Expected 2 id parts".into());
        }

        let id = id_parts[1].parse::<u32>().unwrap();

        let nums_parts = parts[1].split(" | ").collect::<Vec<&str>>();
        if nums_parts.len() != 2 {
            return Err("Expected 2 num parts".into());
        }

        let parse_nums = |s: &str| {
            s.split(" ")
                .filter(|p| !p.is_empty())
                .map(|p| p.parse::<u32>())
                .collect::<Result<HashSet<_>, _>>()
                .unwrap()
        };

        Ok(Card {
            id,
            winning_nums: parse_nums(nums_parts[0]),
            my_nums: parse_nums(nums_parts[1]),
        })
    }
}
