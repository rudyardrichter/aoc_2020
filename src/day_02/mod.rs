use std::convert::TryFrom;
use std::error::Error;

use crate::errors::AOCError;

struct PasswordPolicy {
    min: usize,
    max: usize,
    chr: char,
}

impl TryFrom<String> for PasswordPolicy {
    type Error = Box<dyn Error>;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        let v_0: Vec<&str> = string.splitn(2, "-").collect();
        let min: usize = match v_0.get(0) {
            Some(s) => Ok(s.parse::<usize>()?),
            None => Err(AOCError::new("line missing password policy min")),
        }?;
        let v_1: Vec<&str> = v_0.get(1).unwrap_or(&"").splitn(2, " ").collect();
        let max: usize = match v_1.get(0) {
            Some(s) => Ok(s.parse::<usize>()?),
            None => Err(AOCError::new("line missing password policy max")),
        }?;
        let chr: char = match v_1.get(1) {
            Some(s) => Ok(s.parse::<char>()?),
            None => Err(AOCError::new("line missing password policy chr")),
        }?;
        Ok(PasswordPolicy {
            min: min,
            max: max,
            chr: chr,
        })
    }
}

type Password = String;

struct PasswordRow {
    policy: PasswordPolicy,
    password: Password,
}

impl PasswordRow {
    fn is_valid_1(&self) -> bool {
        let count: usize = self.password.matches(self.policy.chr).count();
        self.policy.min <= count && count <= self.policy.max
    }

    fn is_valid_2(&self) -> bool {
        let chars = self.password.chars().into_iter().collect::<Vec<char>>();
        let a = chars.get(self.policy.min);
        let b = chars.get(self.policy.max);
        (a.is_some() && b.is_some())
            && ((a == Some(&self.policy.chr)) ^ (b == Some(&self.policy.chr)))
    }
}

impl TryFrom<String> for PasswordRow {
    type Error = Box<dyn Error>;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let v: Vec<&str> = s.splitn(2, ":").collect();
        let policy: PasswordPolicy = PasswordPolicy::try_from(v.get(0).unwrap_or(&"").to_string())?;
        let password: Password = v.get(1).unwrap_or(&"").to_string();
        Ok(PasswordRow {
            policy: policy,
            password: password,
        })
    }
}

pub fn part_1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(input
        .lines()
        .map(|line| PasswordRow::try_from(line.to_string()))
        .collect::<Result<Vec<PasswordRow>, _>>()?
        .into_iter()
        .filter(|row| row.is_valid_1())
        .collect::<Vec<PasswordRow>>()
        .len()
        .to_string())
}

pub fn part_2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(input
        .lines()
        .map(|line| PasswordRow::try_from(line.to_string()))
        .collect::<Result<Vec<PasswordRow>, _>>()?
        .into_iter()
        .filter(|row| row.is_valid_2())
        .collect::<Vec<PasswordRow>>()
        .len()
        .to_string())
}

pub fn main(input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(format!(
        "{}\n{}",
        part_1(input.clone())?,
        part_2(input.clone())?
    ))
}
