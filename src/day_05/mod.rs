use std::collections::HashSet;

use crate::errors::AOCError;

struct Seat(usize);

impl From<&str> for Seat {
    fn from(string: &str) -> Self {
        let mut i: usize = 0;
        let mut e: u32 = 0;
        for c in string.chars().rev() {
            if c == 'B' || c == 'R' {
                i += (2 as usize).pow(e);
            }
            e += 1
        }
        Seat(i)
    }
}

pub fn part_1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    input
        .lines()
        .map(|l| Seat::from(l).0)
        .max()
        .ok_or("no input".into())
        .map(|i| i.to_string())
}

pub fn part_2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut seats: HashSet<usize> = HashSet::new();
    for l in input.lines() {
        let s = Seat::from(l);
        if (128..896).contains(&s.0) {
            seats.insert(s.0);
        }
    }
    for i in 128..896 {
        if !seats.contains(&i) {
            return Ok(i.to_string());
        }
    }
    Err(Box::new(AOCError::new("no answer")))
}

pub fn main(input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(format!(
        "{}\n{}",
        part_1(input.clone())?,
        part_2(input.clone())?
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(Seat::from("FBFBBFFRLR").0, 357);
        assert_eq!(Seat::from("BFFFBBFRRR").0, 567);
        Ok(())
    }
}
