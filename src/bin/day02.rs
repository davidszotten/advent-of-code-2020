use anyhow::{anyhow, Error, Result};
use aoc2020::dispatch;
use lazy_static::lazy_static;
use regex::Regex;
use std::convert::TryFrom;

struct Password<'a> {
    min: usize,
    max: usize,
    c: char,
    password: &'a str,
}

impl<'a> Password<'a> {
    fn new(min: usize, max: usize, c: char, password: &'a str) -> Self {
        Self {
            min,
            max,
            c,
            password,
        }
    }

    fn is_valid(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.c).count();
        count >= self.min && count <= self.max
    }

    fn is_valid2(&self) -> Result<bool> {
        let mut chars = self.password.chars();
        let char1 = chars
            .nth(self.min - 1)
            .ok_or(anyhow!("for short for first char"))?;
        let char2 = chars.nth(self.max - self.min - 1).ok_or(anyhow!(format!(
            "`{}` too short for second char ({})",
            self.password, self.max
        )))?;
        Ok((char1 == self.c) ^ (char2 == self.c))
    }
}

impl<'a> TryFrom<&'a str> for Password<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
                // 1-3 b: cdefg
                Regex::new(r"(\d+)-(\d+) (\w): (\w+)")
                    .expect("regex create");
        }
        let caps = RE.captures(s).ok_or(anyhow!("regex mismatch"))?;

        let min: usize = caps[1].parse()?;
        let max: usize = caps[2].parse()?;
        let c: char = caps[3].parse()?;
        // need to use `.get` to get back a Match which keeps the lifetime
        let password: &str = caps
            .get(4)
            .ok_or(anyhow!("regex missing group 4"))?
            .as_str();
        Ok(Self::new(min, max, c, password))
    }
}

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<usize> {
    Ok(input
        .split('\n')
        .map(|s| Password::try_from(s))
        .collect::<Result<Vec<_>>>()?
        .iter()
        .filter(|p| p.is_valid())
        .count())
}

fn part2(input: &str) -> Result<usize> {
    Ok(input
        .split('\n')
        .map(|s| Password::try_from(s).and_then(|p| p.is_valid2()))
        .collect::<Result<Vec<_>>>()?
        .iter()
        .filter(|&v| *v)
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 2);
        Ok(())
    }

    #[test]
    fn test_part1_bad_regex() -> Result<()> {
        assert!(part1("foo").is_err());
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert!(!Password::try_from("1-3 b: cdefg")?.is_valid2()?);
        assert!(!Password::try_from("2-9 c: ccccccccc")?.is_valid2()?);
        assert_eq!(part2(INPUT)?, 1);
        Ok(())
    }

    #[test]
    fn test_part2_too_short() -> Result<()> {
        assert!(part2("1-9 b: cdefg").is_err());
        Ok(())
    }
}
