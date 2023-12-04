use super::Solution;

fn parse_nums(nums: &str) -> Vec<i32> {
    nums.split_whitespace()
        .map(&str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<i32>>()
}

fn part1(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let (win_str, our_str) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let winning_nums = parse_nums(win_str);
        let exp: u32 = parse_nums(our_str)
            .iter()
            .map(|num| winning_nums.contains(num))
            .map(u32::from)
            .sum();
        sum += 2u32.pow(exp - 1) * exp.min(1);
    }
    println!("{sum}");
}

fn part2(input: &str) {
    let mut scratch_cards = Vec::new();
    for line in input.lines() {
        let (win_str, our_str) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        scratch_cards.push((parse_nums(win_str), parse_nums(our_str)));
    }
    let mut copies = vec![1; scratch_cards.len()];
    let mut idx = 0usize;
    while !scratch_cards.is_empty() {
        let (winning_nums, our_nums) = scratch_cards.first().unwrap();
        for i in 0..our_nums
            .iter()
            .map(|num| winning_nums.contains(num))
            .map(usize::from)
            .sum::<usize>()
        {
            copies[idx + i + 1] += copies[idx];
        }
        scratch_cards.remove(0);
        idx += 1;
    }
    println!("{}", copies.iter().sum::<i32>());
}

pub const DAY4: Solution = Solution { part1, part2 };
