use std::{collections::HashMap, ops::Range, str::FromStr};

fn main() {
    let input = include_str!("../input.txt");
    let almanac = Almanac::from_str(input).expect("Failed to parse almanac");

    part1(&almanac);
    part2(&almanac);
}

fn part1(almanac: &Almanac) {
    let mut lowest = u64::MAX;

    for seed in &almanac.seeds {
        lowest = lowest.min(almanac.location_from_seed(*seed));
    }

    println!("Part 1 result: {lowest}");
}

fn part2(almanac: &Almanac) {
    let mut lowest = u64::MAX;

    for seed_range in almanac.seeds.chunks(2) {
        for seed in seed_range[0]..seed_range[0] + seed_range[1] {
            lowest = lowest.min(almanac.location_from_seed(seed));
        }
    }

    println!("Part 2 result: {lowest}");
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    property_maps: HashMap<String, PropertyMap>,
}

impl Almanac {
    fn location_from_seed(&self, mut value: u64) -> u64 {
        let mut property = "seed";

        while property != "location" {
            let map = &self
                .property_maps
                .get(property)
                .expect("Failed to find mapping from property");

            value = map.translate(value);
            property = &map.dest;
        }

        value
    }
}

impl FromStr for Almanac {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split("\n");

        // First line is seeds
        let seeds_line = lines.next().expect("seeds line");
        let seeds = seeds_line
            .split("seeds: ")
            .nth(1)
            .expect("seeds array")
            .split(" ")
            .map(|seed| seed.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .expect("Failed to parse seeds");

        // Skip next line (blank)
        lines.next();
        let mut property_maps = HashMap::new();

        loop {
            let property_map_lines = lines
                .by_ref()
                .take_while(|l| !l.is_empty())
                .collect::<Vec<&str>>();

            if property_map_lines.is_empty() {
                break;
            }

            let property_map = PropertyMap::from_strs(property_map_lines);
            property_maps.insert(property_map.source.clone(), property_map);
        }

        Ok(Almanac {
            seeds,
            property_maps,
        })
    }
}

#[derive(Debug)]
struct PropertyMap {
    source: String,
    dest: String,
    ranges: Vec<RangeMapping>,
}

impl PropertyMap {
    fn translate(&self, value: u64) -> u64 {
        for range in &self.ranges {
            if let Some(translated) = range.translate(value) {
                return translated;
            }
        }

        value
    }

    fn from_strs(lines: Vec<&str>) -> Self {
        let mut properties = lines[0]
            .split(" ")
            .next()
            .expect("properties line")
            .split("-");

        let source = properties.next().expect("Property 1").into();
        let dest = properties.skip(1).next().expect("Property 2").into();

        let mut ranges = Vec::new();

        for line in &lines[1..] {
            let nums = line
                .split(" ")
                .map(|n| n.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()
                .expect("expected array of nums");

            let source_rng = Range {
                start: nums[1],
                end: nums[1] + nums[2],
            };
            let dest_rng = Range {
                start: nums[0],
                end: nums[0] + nums[2],
            };

            ranges.push(RangeMapping {
                source: source_rng,
                dest: dest_rng,
            });
        }

        PropertyMap {
            source,
            dest,
            ranges,
        }
    }
}

#[derive(Debug)]
struct RangeMapping {
    source: Range<u64>,
    dest: Range<u64>,
}
impl RangeMapping {
    fn translate(&self, value: u64) -> Option<u64> {
        if !self.source.contains(&value) {
            return None;
        }

        let diff = value - self.source.start;
        Some(self.dest.start + diff)
    }
}
