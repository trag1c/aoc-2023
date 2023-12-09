use super::Solution;

fn part1(input: &str) {
    fn get_nums(line: &str) -> Vec<i32> {
        line.split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(&str::parse::<i32>)
            .map(Result::unwrap)
            .collect::<Vec<i32>>()
    }
    let (time_str, distance_str) = input.split_once('\n').unwrap();
    let (times, distances) = (get_nums(time_str), get_nums(distance_str));

    let mut product = 1;
    for (time, distance) in times.iter().zip(distances) {
        let f = move |x: i32| (time - x) * x;
        product *= (1..*time)
            .map(|x| f(x) > distance)
            .map(u32::from)
            .sum::<u32>();
    }
    println!("{product}");
}

fn part2(input: &str) {
    fn get_num(line: &str) -> i64 {
        line.split_once(':')
            .unwrap()
            .1
            .replace(' ', "")
            .parse::<i64>()
            .unwrap()
    }
    let (time_str, distance_str) = input.split_once('\n').unwrap();
    let (time, distance) = (get_num(time_str), get_num(distance_str));
    println!(
        "{}",
        (1..time)
            .map(move |x| (time - x) * x > distance)
            .map(u32::from)
            .sum::<u32>()
    );
}

pub const DAY6: Solution = Solution { part1, part2 };
