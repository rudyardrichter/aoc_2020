use std::collections::HashMap;
use std::convert::TryFrom;

use regex::Regex;

use crate::errors::AOCError;

enum Unit {
    Inches,
    Centimeters,
}

struct Height {
    unit: Unit,
    value: usize,
}

impl TryFrom<&str> for Height {
    type Error = AOCError<'static>;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let unit = {
            if string.ends_with("cm") {
                Ok(Unit::Centimeters)
            } else if string.ends_with("in") {
                Ok(Unit::Inches)
            } else {
                Err(AOCError::new("bad units"))
            }
        }?;
        let value = Regex::new(r"\d+")
            .unwrap()
            .find(string)
            .ok_or(AOCError::new("missing height digits"))?
            .as_str()
            .parse::<usize>()
            .map_err(|_| AOCError::new("invalid height"))?;
        Ok(Height {
            unit: unit,
            value: value,
        })
    }
}

struct Passport<'a>(HashMap<&'a str, &'a str>);

impl<'a> TryFrom<&'a str> for Passport<'a> {
    type Error = AOCError<'static>;

    fn try_from(string: &'a str) -> Result<Self, Self::Error> {
        Ok(Passport(
            string
                .clone()
                .split_whitespace()
                .map(|kv| match &kv.split(":").collect::<Vec<&str>>()[..] {
                    &[k, v] => Ok((k.clone(), v.clone())),
                    _ => {
                        let s: &'static str = "";
                        Err(AOCError::new(s))
                    }
                })
                .collect::<Result<HashMap<&str, &str>, AOCError<'static>>>()?,
        ))
    }
}

impl<'a> Passport<'a> {
    fn is_valid_1(&self) -> bool {
        let keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        keys.iter().all(|k| self.0.contains_key(k))
    }

    fn is_valid_2(&self) -> bool {
        // TODO: what is a more elegant method
        // also could do all of these via regex...
        let f = || -> Result<bool, Box<dyn std::error::Error>> {
            let byr = self.0.get("byr").ok_or("")?.parse::<usize>()?;
            let iyr = self.0.get("iyr").ok_or("")?.parse::<usize>()?;
            let eyr = self.0.get("eyr").ok_or("")?.parse::<usize>()?;
            let hgt = Height::try_from(*self.0.get("hgt").ok_or("")?)?;
            let hcl = self.0.get("hcl").ok_or("")?;
            let ecl = self.0.get("ecl").ok_or("")?;
            let pid = self.0.get("pid").ok_or("missing pid")?;
            Ok(self.is_valid_1()
                && (1920..2003).contains(&byr)
                && (2010..2021).contains(&iyr)
                && (2020..2031).contains(&eyr)
                && (match hgt.unit {
                    Unit::Inches => (59..77).contains(&hgt.value),
                    Unit::Centimeters => (150..194).contains(&hgt.value),
                })
                && Regex::new(r"\#[0-9a-f]{6}").unwrap().is_match(hcl)
                && Regex::new(r"amb|blu|brn|gry|grn|hzl|oth")
                    .unwrap()
                    .is_match(ecl)
                && pid.len() == 9)
        };
        match f() {
            Ok(b) => b,
            _ => false,
        }
    }
}

pub fn part_1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(input
        .split("\n\n")
        .try_fold::<usize, _, Result<usize, AOCError>>(0, |acc, ls| {
            Ok(acc + (Passport::try_from(ls)?.is_valid_1() as usize))
        })?
        .to_string())
}

pub fn part_2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(input
        .split("\n\n")
        .try_fold::<usize, _, Result<usize, AOCError>>(0, |acc, ls| {
            Ok(acc + (Passport::try_from(ls)?.is_valid_2() as usize))
        })?
        .to_string())
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

    const TEST_INPUT: &str = concat!(
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n",
        "byr:1937 iyr:2017 cid:147 hgt:183cm\n",
        "\n",
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n",
        "hcl:#cfa07d byr:1929\n",
        "\n",
        "hcl:#ae17e1 iyr:2013\n",
        "eyr:2024\n",
        "ecl:brn pid:760753108 byr:1931\n",
        "hgt:179cm\n",
        "\n",
        "hcl:#cfa07d eyr:2025 pid:166559648\n",
        "iyr:2011 ecl:brn hgt:59in\n",
    );

    #[test]
    fn test_part_1_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(part_1(TEST_INPUT.to_string())?.parse::<usize>()?, 2);
        Ok(())
    }
}
