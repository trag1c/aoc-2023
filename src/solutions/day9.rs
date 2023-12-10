use super::Solution;
use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(&str::parse::<i32>)
                .map(Result::unwrap)
                .collect_vec()
        })
        .collect_vec()
}

fn diffs(vec: &[i32]) -> Vec<i32> {
    vec.iter().tuple_windows().map(|(a, b)| b - a).collect_vec()
}

fn part1(input: &str) {
    let mut sum = 0;
    for history in parse(input) {
        let mut diff_vec = vec![history.clone()];
        let mut curr = history;
        loop {
            let diff = diffs(&curr);
            diff_vec.push(diff.clone());
            if *diff.iter().all_equal_value().unwrap_or(&1) == 0 {
                break;
            }
            curr = diff;
        }
        diff_vec.last_mut().unwrap().push(0);
        for i in (0..diff_vec.len() - 1).rev() {
            let new_val = diff_vec[i].last().unwrap() + diff_vec[i + 1].last().unwrap();
            diff_vec[i].push(new_val);
        }
        sum += diff_vec.first().unwrap().last().unwrap();
    }
    println!("{sum}");
}

fn part2(input: &str) {
    let mut sum = 0;
    for history in parse(input) {
        let mut diff_vec = vec![history.clone()];
        let mut curr = history;
        loop {
            let diff = diffs(&curr);
            diff_vec.push(diff.clone());
            if *diff.iter().all_equal_value().unwrap_or(&1) == 0 {
                break;
            }
            curr = diff;
        }
        diff_vec.last_mut().unwrap().push(0);
        for i in (0..diff_vec.len() - 1).rev() {
            let new_val = diff_vec[i].first().unwrap() - diff_vec[i + 1].first().unwrap();
            diff_vec[i].insert(0, new_val);
        }
        sum += diff_vec.first().unwrap().first().unwrap();
    }
    println!("{sum}");
}

pub const DAY9: Solution = Solution { part1, part2 };
