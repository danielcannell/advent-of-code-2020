use anyhow::{bail, Result};
use clap::{App, Arg};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() -> Result<()> {
    let matches = App::new("advent-of-code-2020")
        .arg(Arg::with_name("day").required(true))
        .get_matches();

    let day_str = matches.value_of("day").unwrap();
    let day: u32 = day_str.parse()?;

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
        _ => bail!("I haven't solved that day yet!"),
    };

    Ok(())
}
