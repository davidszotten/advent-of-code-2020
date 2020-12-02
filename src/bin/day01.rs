use anyhow::{bail, Result};
use aoc2020::dispatch;
use std::collections::HashSet;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<i32> {
    let mut seen = HashSet::new();
    for n in input.split('\n').filter_map(|x| x.parse::<i32>().ok()) {
        let pair = 2020 - n;
        if seen.contains(&pair) {
            return Ok(n * pair);
        }
        seen.insert(n);
    }
    bail!("No match found");
}

fn part2(input: &str) -> Result<i32> {
    let numbers: HashSet<_> = input
        .split('\n')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    for i in numbers.iter() {
        for j in numbers.iter() {
            if j == i {
                continue;
            }
            let missing = 2020 - i - j;
            if numbers.contains(&missing) {
                return Ok(i * j * missing);
            }
        }
    }
    bail!("No match found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1721
979
366
299
675
1456";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 514579);
        Ok(())
    }

    #[test]
    fn test_part1_empty() -> Result<()> {
        assert!(part1("").is_err());
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 241861950);
        Ok(())
    }
}
