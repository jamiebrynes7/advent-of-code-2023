use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use utils::grid::*;

fn main() {
    let input = include_str!("../input.txt");
    let schematic = Schematic::from_str(input).expect("Failed to parse input");

    part1(&schematic);
    part2(&schematic);
}

fn part1(schematic: &Schematic) {
    let result = schematic.find_part_nums().iter().sum::<u32>();
    println!("Part 1 result: {result}");
}

fn part2(schematic: &Schematic) {
    let result = schematic.find_gear_ratios().iter().sum::<u32>();
    println!("Part 2 result: {result}");
}

struct Schematic(Grid<Repr>);

impl Schematic {
    fn find_gear_ratios(&self) -> Vec<u32> {
        let mut symbol_map = HashMap::new();
        let mut collect_symbol = |symbols: &HashSet<Coords>, num: u32| {
            for coords in symbols {
                symbol_map.entry(*coords).or_insert(vec![]).push(num);
            }
        };

        for (y, row) in self.0.rows() {
            let mut current_num = 0;
            let mut adjacent_symbols = HashSet::new();

            for (x, repr) in row.iter().enumerate() {
                let coords = Coords { x, y };
                match repr {
                    Repr::Empty | Repr::Symbol => {
                        if current_num != 0 && !adjacent_symbols.is_empty() {
                            collect_symbol(&adjacent_symbols, current_num);
                        }

                        current_num = 0;
                        adjacent_symbols = HashSet::new();
                    }
                    Repr::Number(n) => {
                        current_num = current_num * 10 + n;
                        adjacent_symbols.extend(self.find_adjacent_symbol(coords).into_iter());
                    }
                }
            }

            if current_num != 0 && !adjacent_symbols.is_empty() {
                collect_symbol(&adjacent_symbols, current_num);
            }
        }

        symbol_map
            .into_iter()
            .filter(|(_, nums)| nums.len() == 2)
            .map(|(_, nums)| nums[0] * nums[1])
            .collect()
    }

    fn find_part_nums(&self) -> Vec<u32> {
        let mut nums = Vec::new();

        for (y, row) in self.0.rows() {
            let mut current_num = 0;
            let mut has_symbol = false;

            for (x, repr) in row.iter().enumerate() {
                let coords = Coords { x, y };
                match repr {
                    Repr::Empty | Repr::Symbol => {
                        if current_num != 0 && has_symbol {
                            nums.push(current_num);
                        }

                        current_num = 0;
                        has_symbol = false;
                    }
                    Repr::Number(n) => {
                        current_num = current_num * 10 + n;
                        has_symbol |= !self.find_adjacent_symbol(coords).is_empty();
                    }
                }
            }

            if current_num != 0 && has_symbol {
                nums.push(current_num);
            }
        }

        nums
    }

    fn find_adjacent_symbol(&self, coords: Coords) -> Vec<Coords> {
        let mut adjacent = vec![];
        for possible in self.0.adjacent_coords(coords) {
            if let Repr::Symbol = self.0[possible] {
                adjacent.push(possible)
            }
        }
        adjacent
    }
}

impl FromStr for Schematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = Vec::new();

        for line in s.split("\n") {
            if line.is_empty() {
                continue;
            }

            let row = line.chars().map(Repr::from).collect();
            inner.push(row);
        }

        Ok(Schematic(Grid::new(inner)))
    }
}

enum Repr {
    Empty,
    Symbol,
    Number(u32),
}

impl From<char> for Repr {
    fn from(value: char) -> Self {
        match value {
            '.' => Repr::Empty,
            '0'..='9' => Repr::Number(value.to_digit(10).unwrap()),
            _ => Repr::Symbol,
        }
    }
}
