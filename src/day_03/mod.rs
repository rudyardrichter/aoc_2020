use std::convert::TryFrom;

use crate::errors::AOCError;

#[derive(Clone)]
struct Trees {
    w: usize,
    h: usize,
    m: Vec<Vec<bool>>,
}

impl Trees {
    fn walk(&self, slope_x: usize, slope_y: usize) -> Result<usize, AOCError<'static>> {
        let mut x: usize = 0;
        let mut c: usize = 0;
        for i in (0..self.h).step_by(slope_y) {
            let p: bool = *self
                .m
                .get(i)
                .ok_or(AOCError::new("y out of bounds"))?
                .get(x)
                .ok_or(AOCError::new("x out of bounds"))?;
            if p {
                c += 1;
            }
            x += slope_x;
            x %= self.w;
        }
        Ok(c)
    }
}

impl TryFrom<String> for Trees {
    type Error = AOCError<'static>;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        let w = string
            .lines()
            .nth(0)
            .ok_or({
                let s: &'static str = "bad input";
                AOCError::new(s)
            })?
            .len();
        let h = string.as_bytes().iter().filter(|&&c| c == b'\n').count();
        let m = string
            .lines()
            .into_iter()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Ok(false),
                        '#' => Ok(true),
                        _ => {
                            let s: &'static str = "unexpected character in tree map";
                            Err(AOCError::new(s))
                        }
                    })
                    .collect::<Result<Vec<bool>, AOCError>>()
            })
            .collect::<Result<Vec<Vec<bool>>, AOCError>>()?;
        Ok(Trees { w: w, h: h, m: m })
    }
}

pub fn part_1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(Trees::try_from(input)?.walk(3, 1)?.to_string())
}

pub fn part_2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let trees = Trees::try_from(input)?;
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    Ok(slopes
        .into_iter()
        .try_fold::<usize, _, Result<usize, AOCError>>(1, |acc, (slope_x, slope_y)| {
            Ok(acc * trees.walk(slope_x, slope_y)?)
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
        "..##.......\n",
        "#...#...#..\n",
        ".#....#..#.\n",
        "..#.#...#.#\n",
        ".#...##..#.\n",
        "..#.##.....\n",
        ".#.#.#....#\n",
        ".#........#\n",
        "#.##...#...\n",
        "#...##....#\n",
        ".#..#...#.#\n",
    );

    #[test]
    fn test_part_1_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(part_1(TEST_INPUT.to_string())?.parse::<usize>()?, 7);
        Ok(())
    }

    #[test]
    fn test_part_2_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(part_2(TEST_INPUT.to_string())?.parse::<usize>()?, 336);
        Ok(())
    }
}
