use anyhow::{anyhow, bail, Result};
use aoc2020::dispatch;
use std::collections::HashSet;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn calculate(pass: &str) -> Result<i32> {
    let mapped: String = pass
        .chars()
        .map(|c| {
            Ok(match c {
                'F' | 'L' => '0',
                'B' | 'R' => '1',
                _ => bail!("Invalid character `{}`", c),
            })
        })
        .collect::<Result<String>>()?;
    Ok(i32::from_str_radix(&mapped, 2)?)
}

fn part1(input: &str) -> Result<i32> {
    input
        .split("\n")
        .map(calculate)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .max()
        .ok_or(anyhow!("no passports"))
}

fn part2(input: &str) -> Result<i32> {
    let numbers = input
        .split("\n")
        .map(calculate)
        .collect::<Result<HashSet<_>>>()?;
    let &min = numbers.iter().min().ok_or(anyhow!("no passes"))?;
    let &max = numbers.iter().max().ok_or(anyhow!("no passes"))?;
    for seat in min..=max {
        if !numbers.contains(&seat)
            && numbers.contains(&(seat - 1))
            && numbers.contains(&(seat + 1))
        {
            return Ok(seat);
        }
    }
    bail!("Seat not found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() -> Result<()> {
        assert_eq!(calculate("FBFBBFFRLR")?, 357);
        assert_eq!(calculate("BFFFBBFRRR")?, 567);
        assert_eq!(calculate("FFFBBBFRRR")?, 119);
        assert_eq!(calculate("BBFFBBFRLL")?, 820);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(
            part1(
                "BFFFBBFRRR
BBFFBBFRLL
FFFBBBFRRR"
            )?,
            820
        );
        Ok(())
    }
}
