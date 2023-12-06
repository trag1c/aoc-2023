use super::Solution;
use itertools::Itertools;
use rayon::prelude::*;
use std::ops::Range;

struct RangeMapping {
    from: Vec<Range<i64>>,
    to: Vec<Range<i64>>,
}

impl RangeMapping {
    fn from_data(data: &[i64]) -> Self {
        let (dest_start, src_start, length) =
            data.iter().take(3).next_tuple::<(_, _, _)>().unwrap();
        Self {
            from: vec![Range {
                start: *src_start,
                end: src_start + length,
            }],
            to: vec![Range {
                start: *dest_start,
                end: dest_start + length,
            }],
        }
    }

    fn get(&self, num: i64) -> i64 {
        for (i, range) in self.from.iter().enumerate() {
            if range.contains(&num) {
                return self.to[i].start + (num - range.start);
            }
        }
        num
    }

    fn merge(self, mut other: Self) -> Self {
        let mut from = self.from.clone();
        let mut to = self.to.clone();
        from.append(&mut other.from);
        to.append(&mut other.to);
        Self { from, to }
    }
}

fn parse(input: &str) -> (Vec<i64>, Vec<RangeMapping>) {
    let mut inputs = input.split("\n\n");
    let seeds = inputs
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(&str::parse::<i64>)
        .map(Result::unwrap)
        .collect::<Vec<i64>>();
    let mappings = inputs
        .map(|i| {
            i.split_once(":\n")
                .unwrap()
                .1
                .lines()
                .map(|line| {
                    let nums = line
                        .split_whitespace()
                        .map(&str::parse::<i64>)
                        .map(Result::unwrap)
                        .collect::<Vec<i64>>();
                    RangeMapping::from_data(&nums)
                })
                .reduce(RangeMapping::merge)
                .unwrap()
        })
        .collect::<Vec<RangeMapping>>();
    (seeds, mappings)
}

fn part1(input: &str) {
    let (seeds, mappings) = parse(input);
    let mut location = i64::MAX;

    for mut seed in seeds {
        for rm in &mappings {
            seed = rm.get(seed);
        }
        location = location.min(seed);
    }
    
    println!("{location}");
}

fn part2(input: &str) {
    let (seeds, mappings) = parse(input);
    let seed_ranges = seeds.chunks(2).map(|c| Range { start: c[0], end: c[0] + c[1] }).collect::<Vec<Range<i64>>>();

    let out = seed_ranges.par_iter().map(|range: &Range<i64>| -> i64 {
        let mut loc = i64::MAX;
        for mut seed in range.clone() {
            for rm in &mappings {
                seed = rm.get(seed);
            }
            loc = loc.min(seed);
        }
        loc
    }).reduce(|| i64::MAX, i64::min);

    println!("{out}");
}

pub const DAY5: Solution = Solution { part1, part2 };
