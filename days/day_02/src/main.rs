use std::str::FromStr;

use utils::parse_lines;

fn main() {
    let input = include_str!("../input.txt");
    let result: Vec<Game> = parse_lines(input).unwrap();

    part1(&result);
    part2(&result);
}

fn part1(games: &[Game]) {
    let full_set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    let result = games
        .iter()
        .filter(|g| g.is_possible(&full_set))
        .map(|g| g.id)
        .sum::<u32>();

    println!("Part 1 result: {result}");
}

fn part2(games: &[Game]) {
    let result = games.iter().map(|g| g.power()).sum::<u32>();
    println!("Part 2 result: {result}");
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn is_possible(&self, full_set: &Set) -> bool {
        self.sets
            .iter()
            .all(|s| full_set.blue >= s.blue && full_set.green >= s.green && full_set.red >= s.red)
    }

    fn power(&self) -> u32 {
        let minimum_set = self
            .sets
            .iter()
            .fold(Set::default(), |acc, elem| acc.minimum_set(elem));

        minimum_set.red * minimum_set.green * minimum_set.blue
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(": ").collect::<Vec<&str>>();

        let id = parts[0].split(" ").last().unwrap().parse::<u32>().unwrap();

        let sets = parts[1]
            .split("; ")
            .map(Set::from_str)
            .collect::<Result<Vec<Set>, _>>()
            .unwrap();

        Ok(Game { id, sets })
    }
}

struct Set {
    blue: u32,
    green: u32,
    red: u32,
}

impl Set {
    fn minimum_set(&self, other: &Set) -> Set {
        Set {
            blue: u32::max(self.blue, other.blue),
            green: u32::max(self.green, other.green),
            red: u32::max(self.red, other.red),
        }
    }
}

impl Default for Set {
    fn default() -> Self {
        Set {
            blue: 0,
            green: 0,
            red: 0,
        }
    }
}

impl FromStr for Set {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Set::default();

        for balls in s.split(", ") {
            let mut parts = balls.split(" ");
            let count = parts
                .next()
                .ok_or("expected to find first part".to_string())?
                .parse::<u32>()
                .map_err(|_| "expected to be number".to_string())?;

            let color = parts
                .next()
                .ok_or("expected to find second part".to_string())?;

            match color {
                "blue" => {
                    set.blue = count;
                }
                "green" => {
                    set.green = count;
                }
                "red" => {
                    set.red = count;
                }
                _ => return Err("unexpected color found".to_string()),
            }
        }

        Ok(set)
    }
}
