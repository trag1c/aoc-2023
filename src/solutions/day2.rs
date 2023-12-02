use super::Solution;

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<(i32, i32, i32)>,
}

impl Game {
    fn from_string(input: &str) -> Self {
        let (game_str, sets_str) = input.split_once(": ").unwrap();
        let id = game_str.split_once(' ').unwrap().1.parse::<i32>().unwrap();
        let mut sets = Vec::new();
        for set_str in sets_str.split("; ") {
            let mut colors = [0; 3];
            for element in set_str.split(", ") {
                let (n, color) = element.split_once(' ').unwrap();
                colors[match color {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => panic!("Invalid color"),
                }] = n.parse::<i32>().unwrap();
            }
            sets.push((colors[0], colors[1], colors[2]));
        }
        Self { id, sets }
    }

    fn is_possible(&self) -> bool {
        !&self
            .sets
            .iter()
            .any(|(r, g, b)| *r > 12 || *g > 13 || *b > 14)
    }

    fn power(self) -> i32 {
        let (mut lr, mut lg, mut lb) = (0, 0, 0);
        for (r, g, b) in &self.sets {
            lr = lr.max(*r);
            lg = lg.max(*g);
            lb = lb.max(*b);
        }
        lr * lg * lb
    }
}

fn part1(input: &str) {
    println!(
        "{}",
        input
            .lines()
            .map(Game::from_string)
            .filter(Game::is_possible)
            .map(|game| game.id)
            .sum::<i32>()
    );
}

fn part2(input: &str) {
    println!(
        "{}",
        input
            .lines()
            .map(Game::from_string)
            .map(Game::power)
            .sum::<i32>()
    );
}

pub const DAY2: Solution = Solution { part1, part2 };
