use super::Solution;

fn part1(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let mut char_idxs = line.chars().filter(char::is_ascii_digit);
        let first = char_idxs.next().unwrap();
        let last = char_idxs.last().unwrap_or(first);
        sum += format!("{first}{last}").parse::<i32>().unwrap();
    }
    println!("{sum}");
}

const NUMS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn word_to_num(word: &str) -> i32 {
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => word.parse().unwrap(),
    }
}

fn find_nums(line: &str) -> Vec<i32> {
    let mut nums = Vec::new();
    let mut max_num_idx = 0;
    'main: loop {
        let chunk = &line[max_num_idx..(max_num_idx + 5).min(line.len())];
        if chunk.is_empty() {
            break;
        }
        for num in NUMS {
            if chunk.starts_with(num) {
                nums.push(word_to_num(num));
                max_num_idx += (num.len() - 1).max(1);
                continue 'main;
            }
        }
        max_num_idx += 1;
    }
    nums
}

fn part2(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let char_idxs = find_nums(line);
        let first = char_idxs.first().unwrap();
        let last = char_idxs.last().unwrap_or(first);
        sum += format!("{first}{last}").parse::<i32>().unwrap();
    }
    println!("{sum}");
}

pub const DAY1: Solution = Solution { part1, part2 };
