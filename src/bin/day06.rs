use anyhow::Result;
use aoc2020::dispatch;
use reduce::Reduce;
use std::collections::HashSet;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<usize> {
    Ok(input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<HashSet<_>>()
        })
        .map(|s| s.len())
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    Ok(input
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|p| p.chars().collect::<HashSet<_>>())
                .reduce(|a, b| a.intersection(&b).cloned().collect())
        })
        .map(|s| s.expect("empty group").len())
        .sum())
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
