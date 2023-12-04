use std::collections::HashMap;

use super::Solution;

fn find_spans(line: &str) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut start = None;
    for (i, c) in line.chars().enumerate() {
        if !c.is_ascii_digit() {
            if start.is_some() {
                spans.push((start.unwrap(), i));
                start = None;
            }
        } else if start.is_none() {
            start = Some(i);
        }
    }
    if let Some(start) = start {
        spans.push((start, line.len()));
    }
    spans
}

fn find_neighbor_indexes(
    span: (usize, usize),
    line_idx: i32,
    line_len: i32,
    line_count: i32,
) -> Vec<i32> {
    let mut neighbors = Vec::new();
    let start = span.0 as i32;
    let end = span.1 as i32;
    if start != 0 {
        neighbors.push((line_len + 1) * line_idx + start - 1);
    }
    if end != line_len - 1 {
        neighbors.push((line_len + 1) * line_idx + end);
    }
    for i in 0.max(start - 1)..(line_len - 1).min(end + 1) {
        if line_idx != 0 {
            neighbors.push((line_len + 1) * (line_idx - 1) + i);
        }
        if line_idx != line_count - 1 {
            neighbors.push((line_len + 1) * (line_idx + 1) + i);
        }
    }
    neighbors
}

fn part1(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    let line_len = input.find('\n').unwrap();
    let line_count = input.len() / line_len;
    let mut sum = 0;
    for (i, line) in lines.iter().enumerate() {
        for span in find_spans(line) {
            if find_neighbor_indexes(span, i as i32, line_len as i32, line_count as i32)
                .iter()
                .map(|i| input.as_bytes()[*i as usize] as char)
                .any(|ch| "*/%=@&#-+$".contains(ch))
            {
                sum += line[span.0..span.1].parse::<i32>().unwrap();
            }
        }
    }
    println!("{sum}");
}

fn part2(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    let line_len = input.find('\n').unwrap();
    let line_count = input.len() / line_len;
    let mut sum = 0;
    let mut potential_gears = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        for span in find_spans(line) {
            if let Some(gear_index) =
                find_neighbor_indexes(span, i as i32, line_len as i32, line_count as i32)
                    .iter()
                    .find(|x| input.as_bytes()[**x as usize] as char == '*')
            {
                if let Some(stored_gear) = potential_gears.get(gear_index) {
                    sum += line[span.0..span.1].parse::<i32>().unwrap() * stored_gear;
                    potential_gears.remove(gear_index);
                } else {
                    potential_gears.insert(*gear_index, line[span.0..span.1].parse::<i32>().unwrap());
                }
            }
        }
    }
    println!("{sum}");
}

pub const DAY3: Solution = Solution { part1, part2 };
