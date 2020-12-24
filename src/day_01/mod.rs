use std::collections::HashSet;

use crate::errors::AOCError;

pub fn part_1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let entries: HashSet<isize> = input
        .lines()
        .map(|line| line.parse::<isize>())
        .collect::<Result<HashSet<isize>, std::num::ParseIntError>>()?;
    let differences: HashSet<isize> = entries.iter().map(|n| 2020 - n).collect();
    entries
        .into_iter()
        .filter(|n| differences.get(n).is_some())
        .nth(0)
        .ok_or(AOCError::new("no result found").into())
        .map(|n| (n * (2020 - n)).to_string())
}

pub fn part_2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let entries: HashSet<isize> = input
        .lines()
        .map(|line| line.parse::<isize>())
        .collect::<Result<HashSet<isize>, std::num::ParseIntError>>()?;
    entries
        .iter()
        .map(|&n| {
            (
                n,
                entries
                    .iter()
                    .map(|&m| 2020 - m - n)
                    .collect::<HashSet<isize>>()
                    .intersection(&entries)
                    .cloned()
                    .collect::<HashSet<isize>>(),
            )
        })
        .filter(|(_, ds)| ds.len() > 0)
        .nth(0)
        .ok_or(AOCError::new("no result found").into())
        .map(|(n, ds)| n * ds.into_iter().product::<isize>())
        .map(|i| i.to_string())
}

pub fn main(input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(format!(
        "{}\n{}",
        part_1(input.clone())?,
        part_2(input.clone())?
    ))
}
