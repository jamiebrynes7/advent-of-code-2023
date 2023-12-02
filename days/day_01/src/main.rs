use std::str::FromStr;

use utils::parse_lines;

fn main() {
    let input = include_str!("./data.txt");
    let lines: Vec<CalibrationLine> = parse_lines(input).unwrap();
    part1(&lines);
    part2(&lines);
}

struct CalibrationLine(String);

impl FromStr for CalibrationLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

impl CalibrationLine {
    fn calibration_value(&self) -> Result<u32, String> {
        let mut first_num = None;
        let mut last_num = None;

        for c in self.0.chars() {
            if !c.is_digit(10) {
                continue;
            }

            let digit = c
                .to_digit(10)
                .ok_or("Failed to convert into digit".to_string())?;

            first_num = first_num.or(Some(digit));
            last_num = Some(digit);
        }

        if first_num.is_none() || last_num.is_none() {
            return Err("Failed to find either first or last digit".to_string());
        }

        Ok(first_num.unwrap() * 10 + last_num.unwrap())
    }

    fn condense(&self) -> CalibrationLine {
        // Scan through character by character, keeping a buffer of up to 5 chars.
        // If the buffer spells out a number, we collapse that into a number and then
        // remove the characters except the last one from the buffer.

        let find_digit = |chars: &[char]| match chars {
            ['o', 'n', 'e', ..] => Some(('1', 3)),
            ['t', 'w', 'o', ..] => Some(('2', 3)),
            ['t', 'h', 'r', 'e', 'e'] => Some(('3', 5)),
            ['f', 'o', 'u', 'r', ..] => Some(('4', 4)),
            ['f', 'i', 'v', 'e', ..] => Some(('5', 4)),
            ['s', 'i', 'x', ..] => Some(('6', 3)),
            ['s', 'e', 'v', 'e', 'n'] => Some(('7', 5)),
            ['e', 'i', 'g', 'h', 't'] => Some(('8', 5)),
            ['n', 'i', 'n', 'e', ..] => Some(('9', 4)),
            _ => None,
        };

        let mut result: Vec<char> = Vec::new();
        let mut buffer: Vec<char> = Vec::new();

        for c in self.0.chars() {
            buffer.push(c);

            if buffer.len() > 5 {
                result.push(buffer.remove(0));
            }

            if let Some((digit, length)) = find_digit(&buffer) {
                result.push(digit);
                buffer.drain(0..length - 1);
            }
        }

        while buffer.len() > 0 {
            if let Some((digit, length)) = find_digit(&buffer) {
                result.push(digit);
                buffer.drain(0..length - 1);
            } else {
                result.push(buffer.remove(0));
            }
        }

        let res = result.iter().collect::<String>();
        // println!("Before {}, after {}", self.0, res);
        CalibrationLine(res)
    }
}

fn part1(lines: &[CalibrationLine]) {
    let result = lines
        .iter()
        .map(|cl| cl.calibration_value())
        .collect::<Result<Vec<u32>, _>>()
        .unwrap()
        .iter()
        .sum::<u32>();

    println!("Part 1: {result}");
}

fn part2(lines: &[CalibrationLine]) {
    let result = lines
        .iter()
        .map(|cl| cl.condense().calibration_value())
        .collect::<Result<Vec<u32>, _>>()
        .unwrap()
        .iter()
        .sum::<u32>();

    println!("Part 2: {result}");
}
