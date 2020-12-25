use regex::Regex;
use std::collections::HashMap;

use crate::errors::AOCError;

type Bags<'a> = HashMap<&'a str, HashMap<&'a str, usize>>;

fn contains_bags(bags: &Bags, name: &str, goal: &str) -> bool {
    bags.get(name)
        .map(|contained| {
            contained.contains_key(goal) || contained.keys().any(|k| contains_bags(bags, k, goal))
        })
        .unwrap_or(false)
}

fn bags_from_input<'a>(input: &'a String) -> Result<Bags, Box<dyn std::error::Error>> {
    let mut bags: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
    let bag_outside_regex = Regex::new(r"^(\w+ \w+)").unwrap();
    let bag_inside_regex = Regex::new(r"(\d) (\w+ \w+)").unwrap();
    for l in input.lines() {
        let e = AOCError::new("parse error"); // abbreviation
        let outside: &str = bag_outside_regex
            .captures(l)
            .ok_or(e.clone())?
            .get(0)
            .ok_or(e.clone())?
            .as_str();
        let mut bags_inside: HashMap<&str, usize> = HashMap::new();
        for c in bag_inside_regex.captures_iter(l) {
            let n: usize = c.get(1).ok_or(e.clone())?.as_str().parse()?;
            let b: &str = c.get(2).ok_or(e.clone())?.as_str();
            bags_inside.insert(b, n);
        }
        bags.insert(outside, bags_inside);
    }
    Ok(bags)
}

// TODO: come back to this with arena + doubly-linked tree
pub fn part_1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let bags: Bags = bags_from_input(&input)?;
    Ok(bags
        .keys()
        .filter(|k| contains_bags(&bags, k, "shiny gold"))
        .count()
        .to_string())
}

fn count_bags(bags: &Bags, start: &str) -> usize {
    bags.get(start)
        .map(|contained| {
            contained
                .iter()
                .map(|(bag, count)| count + count * count_bags(bags, bag))
                .sum()
        })
        .unwrap_or(1)
}

pub fn part_2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let bags: Bags = bags_from_input(&input)?;
    Ok(count_bags(&bags, "shiny gold").to_string())
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

    const TEST_INPUT_1: &str = concat!(
        "light red bags contain 1 bright white bag, 2 muted yellow bags.\n",
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n",
        "bright white bags contain 1 shiny gold bag.\n",
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n",
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n",
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n",
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n",
        "faded blue bags contain no other bags.\n",
        "dotted black bags contain no other bags.\n"
    );

    #[test]
    fn test_part_1_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(part_1(TEST_INPUT_1.to_string())?.parse::<usize>()?, 4);
        Ok(())
    }

    const TEST_INPUT_2: &str = concat!(
        "shiny gold bags contain 2 dark red bags.\n",
        "dark red bags contain 2 dark orange bags.\n",
        "dark orange bags contain 2 dark yellow bags.\n",
        "dark yellow bags contain 2 dark green bags.\n",
        "dark green bags contain 2 dark blue bags.\n",
        "dark blue bags contain 2 dark violet bags.\n",
        "dark violet bags contain no other bags.\n",
    );

    #[test]
    fn test_part_2_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(part_2(TEST_INPUT_1.to_string())?.parse::<usize>()?, 32);
        assert_eq!(part_2(TEST_INPUT_2.to_string())?.parse::<usize>()?, 126);
        Ok(())
    }
}
