use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    let sequences: Vec<Sequence> = utils::parse_lines(input).unwrap();

    solve(&sequences);
}

fn solve(sequences: &[Sequence]) {
    let (part2, part1): (Vec<i64>, Vec<i64>) = sequences.iter().map(|s| s.extrapolate()).unzip();

    let part1_result = part1.iter().sum::<i64>();
    println!("Part 1 result {part1_result}");

    let part2_result = part2.iter().sum::<i64>();
    println!("Part 2 result {part2_result}");
}

struct Sequence(Vec<i64>);

impl Sequence {
    fn extrapolate(&self) -> (i64, i64) {
        let mut sequences = vec![self.0.clone()];

        // Descend the values
        loop {
            let last = sequences.last().unwrap();

            if last.iter().all(|v| *v == 0) {
                break;
            }

            let next = last
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            sequences.push(next);
        }

        let mut next = 0;
        let mut prev = 0;
        for sequence in sequences.iter().rev() {
            next += sequence.last().unwrap();
            prev = sequence.first().unwrap() - prev;
        }

        (prev, next)
    }
}

impl FromStr for Sequence {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split(" ")
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        Ok(Sequence(nums))
    }
}
