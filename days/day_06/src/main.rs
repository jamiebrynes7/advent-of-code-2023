fn main() {
    let input = parse_input(include_str!("../input.txt"));
    part1(&input);
    part2(&input);
}

fn part1(races: &Vec<Race>) {
    let total = races
        .iter()
        .map(|r| r.count_possible_ways())
        .product::<u64>();

    println!("Part 1 result: {total}");
}

fn part2(races: &Vec<Race>) {
    // Dumb way to combine them, stringify and then reparse.
    let time = races
        .iter()
        .map(|r| r.duration.to_string())
        .fold(String::new(), |acc, elem| acc + &elem)
        .parse::<u64>()
        .expect("Expected number");

    let dist = races
        .iter()
        .map(|r| r.record_distance.to_string())
        .fold(String::new(), |acc, elem| acc + &elem)
        .parse::<u64>()
        .expect("Expected number");

    let result = Race {
        duration: time,
        record_distance: dist,
    }
    .count_possible_ways();

    println!("Part 2 result: {result}");
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.split("\n");
    let time_line = lines.next().expect("Failed to read time line");
    let distance_line = lines.next().expect("Failed to read distance line");

    let times = time_line
        .split(" ")
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse times");

    let distances = distance_line
        .split(" ")
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse distances");

    assert!(times.len() == distances.len());

    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race {
            duration: time,
            record_distance: distance,
        })
        .collect()
}

struct Race {
    duration: u64,
    record_distance: u64,
}

impl Race {
    fn count_possible_ways(&self) -> u64 {
        // Effectively solving quadratic formula where a = -1, b = duration, c = -record_distance
        let base = self.duration as f64 / 2.0;
        let offset = 0.5 * (self.duration.pow(2) as f64 - 4.0 * self.record_distance as f64).sqrt();

        let start = (1.0 + base - offset).trunc() as u64;
        let end = (base + offset - 1.0).ceil() as u64;

        1 + end - start
    }
}
