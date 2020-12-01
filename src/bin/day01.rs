use aoc2020::{dispatch, Result};
use std::collections::HashSet;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
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
    Ok(0)
}

fn part2(input: &str) -> Result<i32> {
    let numbers: Vec<_> = input
        .split('\n')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    for i in numbers.iter() {
        for j in numbers.iter() {
            for k in numbers.iter() {
                if i + j + k == 2020 {
                    return Ok(i * j * k);
                }
            }
        }
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(
            part1(
                "1721
979
366
299
675
1456"
            )?,
            514579
        );
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            part2(
                "1721
979
366
299
675
1456"
            )?,
            241861950
        );
        Ok(())
    }
}
