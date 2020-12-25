use crate::errors::AOCError;

fn xmas_1(ns: &Vec<usize>, window: usize) -> Option<usize> {
    for (i, &n) in ns.iter().enumerate() {
        if i < window {
            continue;
        }
        let mut valid = false;
        let check_slice = ns.get((i - window)..i)?;
        'check: for (j, a) in check_slice.iter().enumerate() {
            for (k, b) in check_slice.iter().enumerate() {
                if j != k && a + b == n {
                    valid = true;
                    break 'check;
                }
            }
        }
        if !valid {
            return Some(n);
        }
    }
    None
}

fn xmas_2(ns: &Vec<usize>, invalid: usize) -> Option<usize> {
    let mut sums: Vec<Vec<usize>> = vec![vec![0; ns.len()]; ns.len()];
    // element (i, j) in sums should be ns[i..j].sum()
    //     sums[0][j] == sum of first j numbers in ns
    //     sums[i][i] == ns[i]
    //     sums[i][i+1] == ns[i] + ns[i+i]
    for i in 0..ns.len() {
        sums[i][i] = ns[i]
    }
    for i in 0..ns.len() {
        for j in i + 1..ns.len() {
            sums[i][j] = sums[i][j - 1] + ns[j];
            if sums[i][j] == invalid {
                return Some(ns[i..j].iter().min()? + ns[i..j].iter().max()?);
            }
        }
    }
    None
}

pub fn part_1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let ns = input
        .lines()
        .map(|l| l.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?;
    Ok(xmas_1(&ns, 25)
        .ok_or(AOCError::new("no answer found"))?
        .to_string())
}

pub fn part_2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let ns = input
        .lines()
        .map(|l| l.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?;
    let invalid = xmas_1(&ns, 25).ok_or(AOCError::new("no invalid number found"))?;
    Ok(xmas_2(&ns, invalid)
        .ok_or(AOCError::new("no answer found"))?
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

    #[test]
    fn test_xmas_1() {
        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(xmas_1(&input, 5).unwrap(), 127);
    }

    #[test]
    fn test_part_2_example() {
        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(xmas_2(&input, 127).unwrap(), 62);
    }
}
