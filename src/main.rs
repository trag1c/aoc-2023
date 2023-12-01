mod argv;
mod solutions;

use argv::{parse_args, quit_if, Part};
use solutions::*;

const SOLUTIONS: [Solution; 0] = [];

fn get_solution(day: u8, part: Part) -> fn(String) {
    use Part::*;
    SOLUTIONS
        .get((day - 1) as usize)
        .map(|sol| match part {
            One => sol.part1,
            Two => sol.part2,
        })
        .unwrap_or(|_| println!("Not implemented"))
}

fn load_data(day: u8) -> String {
    let path = format!("inputs/day{day}.txt");
    let contents = std::fs::read_to_string(path.clone());
    quit_if(contents.is_err(), format!("Could not read file {path}"));
    contents.unwrap()
}

fn main() {
    let (day, part) = parse_args();
    let data = load_data(day);
    get_solution(day, part)(data);
}
