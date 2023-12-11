use super::Solution;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn inv(self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

fn parse(input: &str) -> ((usize, usize), Vec<Vec<Vec<Dir>>>) {
    let maze = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    'S' => vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right],
                    '|' => vec![Dir::Up, Dir::Down],
                    'J' => vec![Dir::Up, Dir::Left],
                    'L' => vec![Dir::Up, Dir::Right],
                    '7' => vec![Dir::Down, Dir::Left],
                    'F' => vec![Dir::Down, Dir::Right],
                    '-' => vec![Dir::Left, Dir::Right],
                    '.' => vec![],
                    _ => panic!("Unknown character {ch}"),
                })
                .collect_vec()
        })
        .collect_vec();
    for (i, row) in maze.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col.len() == 4 {
                return ((i, j), maze);
            }
        }
    }
    panic!("No starting point found");
}

fn move_index(pos: (usize, usize), dir: Dir) -> Option<(usize, usize)> {
    Some(match dir {
        Dir::Up => (pos.0.checked_sub(1)?, pos.1),
        Dir::Down => (pos.0.checked_add(1)?, pos.1),
        Dir::Left => (pos.0, pos.1.checked_sub(1)?),
        Dir::Right => (pos.0, pos.1.checked_add(1)?),
    })
}

fn index(maze: &[Vec<Vec<Dir>>], pos: (usize, usize)) -> Vec<Dir> {
    maze[pos.0][pos.1].clone()
}

fn vec_rm_val<T: Eq>(vec: &mut Vec<T>, val: &T) {
    // fn vec_rm_val(vec: &mut Vec<Dir>, val: &Dir) {
    // println!("{vec:?} {val:?}");
    if let Some(val) = vec.iter().position(|x| x == val) {
        vec.remove(val);
    }
}

fn find_initial_dir(start_pos: (usize, usize), maze: &[Vec<Vec<Dir>>]) -> Dir {
    let mut possible_dirs = vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right];
    while possible_dirs.len() > 2 {
        let mut rm_idx = None;
        for dir in &possible_dirs {
            let mv_idx = move_index(start_pos, *dir);
            if mv_idx.is_none() || !index(maze, mv_idx.unwrap()).contains(dir) {
                rm_idx = Some(possible_dirs.iter().position(|x| x == dir).unwrap());
                break;
            }
        }
        if let Some(rm_idx) = rm_idx {
            possible_dirs.remove(rm_idx);
        }
    }
    *possible_dirs.first().unwrap()
}

fn part1(input: &str) {
    let (start_pos, maze) = parse(input);

    let mut steps = 0;
    let mut pos = start_pos;

    let mut dir = find_initial_dir(start_pos, &maze);

    while steps == 0 || pos != start_pos {
        steps += 1;
        pos = move_index(pos, dir).unwrap();
        let mut next = index(&maze, pos).clone();
        vec_rm_val(&mut next, &dir.inv());
        dir = *next.first().unwrap();
    }

    steps /= 2;
    println!("{steps}");
}

fn part2(input: &str) {
    let (start_pos, maze) = parse(input);

    let mut path = Vec::new();
    let mut pos = start_pos;

    let mut dir = find_initial_dir(start_pos, &maze);

    while path.is_empty() || pos != start_pos {
        pos = move_index(pos, dir).unwrap();
        path.push(pos);
        let mut next = index(&maze, pos).clone();
        vec_rm_val(&mut next, &dir.inv());
        dir = *next.first().unwrap();
    }

    let (m, n) = path
        .iter()
        .zip(path[1..].iter().chain(path.iter()))
        .map(|((y0, x0), (y1, x1))| (x0 * y1, x1 * y0))
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));

    println!("{}", (2 + m.max(n) - m.min(n) - path.len()) / 2);
}

pub const DAY10: Solution = Solution { part1, part2 };
