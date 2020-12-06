use anyhow::{anyhow, Result};
use aoc2020::dispatch;
use reduce::Reduce;
use std::collections::HashSet;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

type Answers = HashSet<char>;

fn collate(input: &str, set_fn: &dyn Fn(&Answers, &Answers) -> Answers) -> Result<usize> {
    Ok(input
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|p| p.chars().collect::<HashSet<_>>())
                .reduce(|a, b| set_fn(&a, &b))
        })
        .collect::<Option<Vec<_>>>()
        .ok_or(anyhow!("empty group"))?
        .iter()
        .map(|s| s.len())
        .sum())
}

fn part1(input: &str) -> Result<usize> {
    collate(input, &|a, b| a | b)
}

fn part2(input: &str) -> Result<usize> {
    collate(input, &|a, b| a & b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 11);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 6);
        Ok(())
    }
}
