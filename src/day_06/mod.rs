use bitvec::prelude::*;

fn str_to_bits(s: &str) -> BitArray {
    let mut a = bitarr![0; 26];
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            a.set(c as usize - 'a' as usize, true)
        }
    }
    a
}

pub fn part_1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(input
        .split("\n\n")
        .map(|g| str_to_bits(g).count_ones())
        .sum::<usize>()
        .to_string())
}

pub fn part_2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(input
        .split("\n\n")
        .map(|g| {
            g.trim_end()
                .split("\n")
                .map(|l| str_to_bits(l))
                .fold(bitarr![1; 26], |acc, a| acc & a)
                .count_ones()
        })
        .sum::<usize>()
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

    const TEST_INPUT: &str = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb\n";

    #[test]
    fn test_part_1_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(part_1(TEST_INPUT.to_string())?.parse::<usize>()?, 11);
        Ok(())
    }

    #[test]
    fn test_part_2_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(part_2(TEST_INPUT.to_string())?.parse::<usize>()?, 6);
        Ok(())
    }
}
