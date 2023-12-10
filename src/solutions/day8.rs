use super::Solution;
use itertools::Itertools;
use std::collections::HashMap;

fn parse(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
    let (move_order, instructions) = input.split_once("\n\n").unwrap();
    for instruction in instructions.lines() {
        let (key, values) = instruction.split_once(" = ").unwrap();
        network.insert(
            key,
            values
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(", ")
                .unwrap(),
        );
    }
    (move_order, network)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    match a.max(b) {
        0 => 0,
        _ => (a * b) / gcd(a, b),
    }
}

fn part1(input: &str) {
    let (move_order, network) = parse(input);
    let mut current = "AAA";
    let mut steps = 0;
    let mut move_iter = move_order.chars().cycle();
    while current != "ZZZ" {
        let mv = move_iter.next().unwrap();
        let options = network.get(current).unwrap();
        current = if mv == 'L' { options.0 } else { options.1 };
        steps += 1;
    }
    println!("{steps}");
}

fn part2(input: &str) {
    let (move_order, network) = parse(input);
    let current = network
        .keys()
        .filter_map(|k| if k.ends_with('A') { Some(*k) } else { None })
        .collect_vec();
    let mut steps = [0u64; 6];
    for (i, v) in current.iter().enumerate() {
        let mut c = *v;
        let mut s = 0;
        let mut move_iter = move_order.chars().cycle();
        while !c.ends_with('Z') {
            let mv = move_iter.next().unwrap();
            let (left, right) = *network.get(c).unwrap();
            c = if mv == 'L' { left } else { right };
            s += 1;
        }
        steps[i] = s;
    }
    println!("{}", steps.iter().copied().reduce(lcm).unwrap());
}

pub const DAY8: Solution = Solution { part1, part2 };
