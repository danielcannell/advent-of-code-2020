use std::env;
use std::process::exit;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Error: incorrect number of arguments");
        eprintln!();
        eprintln!("Usage:");
        eprintln!("    advent-of-code-2020 <day>");
        exit(1);
    }

    if args[1] == "--help" || args[1] == "-h" {
        println!("Usage:");
        println!("    advent-of-code-2020 <day>");
        return;
    }

    let day: u32 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Invalid day");
        exit(1);
    });

    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        5 => day5::solve(),
        6 => day6::solve(),
        7 => day7::solve(),
        8 => day8::solve(),
        9 => day9::solve(),
        10 => day10::solve(),
        11 => day11::solve(),
        12 => day12::solve(),
        13 => day13::solve(),
        14 => day14::solve(),
        15 => day15::solve(),
        16 => day16::solve(),
        17 => day17::solve(),
        18 => day18::solve(),
        19 => day19::solve(),
        20 => day20::solve(),
        21 => day21::solve(),
        22 => day22::solve(),
        23 => day23::solve(),
        24 => day24::solve(),
        25 => day25::solve(),

        _ => {
            eprintln!("I haven't solved that day yet!");
            exit(1);
        }
    };
}
