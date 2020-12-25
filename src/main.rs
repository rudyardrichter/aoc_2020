use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::result::Result;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod errors;
mod types;

use errors::AOCError;

// this needs to be a procedural macro...
fn solve(day: usize, part_opt: Option<usize>) -> Result<String, Box<dyn Error>> {
    let input: String = fs::read_to_string(format!("inputs/day_{:02}/input", day))
        .map_err(|_| AOCError::new("input file missing"))?;
    if let Some(part) = part_opt {
        match part {
            1 => match day {
                1 => day_01::part_1(input),
                2 => day_02::part_1(input),
                3 => day_03::part_1(input),
                4 => day_04::part_1(input),
                5 => day_05::part_1(input),
                6 => day_06::part_1(input),
                7 => day_07::part_1(input),
                8 => day_08::part_1(input),
                9 => day_09::part_1(input),
                _ => Err(AOCError::new("not implemented").into()),
            },
            2 => match day {
                1 => day_01::part_2(input),
                2 => day_02::part_2(input),
                3 => day_03::part_2(input),
                4 => day_04::part_2(input),
                5 => day_05::part_2(input),
                6 => day_06::part_2(input),
                7 => day_07::part_2(input),
                8 => day_08::part_2(input),
                9 => day_09::part_2(input),
                _ => Err(AOCError::new("not implemented").into()),
            },
            _ => Err(AOCError::new("bad part").into()),
        }
    } else {
        match day {
            1 => day_01::main(input),
            2 => day_02::main(input),
            3 => day_03::main(input),
            4 => day_04::main(input),
            5 => day_05::main(input),
            6 => day_06::main(input),
            7 => day_07::main(input),
            8 => day_08::main(input),
            9 => day_09::main(input),
            _ => Err(AOCError::new("not implemented").into()),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let day: usize = env::args()
        .nth(1)
        .ok_or(io::Error::new(
            io::ErrorKind::InvalidInput,
            "missing argument for day",
        ))?
        .parse::<usize>()?;
    let part: Option<usize> = env::args()
        .nth(2)
        .map(|p| p.parse::<usize>().ok())
        .flatten();
    let answer: String = solve(day, part)?;
    println!("{}", answer);
    Ok(())
}
