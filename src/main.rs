mod argv;
mod solutions;

use argv::{parse_args, quit_if, Part};
use solutions::*;
use std::time::Instant;

const SOLUTIONS: [Solution; 3] = [DAY1, DAY2, DAY3];

fn get_solution(day: u8, part: &Part) -> fn(&str) {
    use Part::*;
    SOLUTIONS.get((day - 1) as usize).map_or(
        |_| println!("Not implemented"),
        |sol| match part {
            One => sol.part1,
            Two => sol.part2,
        },
    )
}

fn load_data(day: u8) -> String {
    let path = format!("inputs/day{day}.txt");
    let contents = std::fs::read_to_string(path.clone());
    quit_if(
        contents.is_err(),
        format!("Could not read file {path}").as_str(),
    );
    contents.unwrap()
}

fn main() {
    let (day, part) = parse_args();

    let now = Instant::now();
    let data = load_data(day);
    println!("Loaded data in {}µs", now.elapsed().as_micros());

    let now = Instant::now();
    print!("Solution: ");
    get_solution(day, &part)(&data);
    println!("Found in {}µs", now.elapsed().as_micros());
}
