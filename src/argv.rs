#[derive(Debug)]
pub enum Part {
    One,
    Two,
}

pub fn quit_if(cond: bool, msg: &str) {
    if cond {
        println!("{msg}");
        std::process::exit(1);
    }
}

fn get_arg(n: usize) -> Option<u8> {
    if let Some(arg) = std::env::args().nth(n) {
        arg.parse::<u8>().ok()
    } else {
        Some(0)
    }
}

pub fn parse_args() -> (u8, Part) {
    let day = get_arg(1);
    quit_if(day.is_none(), "Day is not an integer");
    let day = day.unwrap();
    quit_if(
        !(1..=25).contains(&day),
        format!("Invalid day ({day})").as_str(),
    );

    let part = get_arg(2);
    quit_if(part.is_none(), "Part is not an integer");
    quit_if(
        !(1..=2).contains(&part.unwrap()),
        format!("Invalid part ({})", part.unwrap()).as_str(),
    );

    let part = match part.unwrap() {
        1 => Part::One,
        2 => Part::Two,
        _ => unreachable!(),
    };

    (day, part)
}
