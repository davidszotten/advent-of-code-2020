use anyhow::{anyhow, bail, Result};
use aoc2020::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn valid(number: i64, preceeding: &[i64]) -> bool {
    for i in 0..preceeding.len() {
        for j in i..preceeding.len() {
            let m = preceeding[i];
            let n = preceeding[j];
            if m + n == number {
                return true;
            }
        }
    }
    false
}

fn find_invalid(numbers: &[i64], pre_length: usize) -> Result<i64> {
    for index in pre_length..numbers.len() {
        let number = numbers[index];
        if !valid(number, &numbers[index - pre_length..index]) {
            return Ok(number);
        }
    }
    bail!("didn't find invalid entry");
}

fn parse(input: &str) -> Result<Vec<i64>> {
    input
        .split('\n')
        .map(|r| r.parse::<i64>().map_err(|_| anyhow!("not a number")))
        .collect()
}

fn find_run(numbers: &[i64], invalid: i64) -> Result<i64> {
    for start in 0..numbers.len() {
        for length in 2..(numbers.len() - start) {
            let interval = &numbers[start..start + length];
            let sum: i64 = interval.iter().sum();
            if sum == invalid {
                let min = interval.iter().min().expect("min");
                let max = interval.iter().max().expect("max");
                return Ok(min + max);
            } else if sum > invalid {
                break;
            }
        }
    }
    bail!("no run found");
}

fn part1(input: &str) -> Result<i64> {
    let numbers = parse(input)?;
    find_invalid(&numbers, 25)
}

fn part2(input: &str) -> Result<i64> {
    let numbers = parse(input)?;
    let invalid = find_invalid(&numbers, 25)?;
    find_run(&numbers, invalid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_check() -> Result<()> {
        assert_eq!(find_invalid(&parse(INPUT)?, 5,)?, 127);
        Ok(())
    }

    #[test]
    fn test_find() -> Result<()> {
        assert_eq!(find_run(&parse(INPUT)?, 127)?, 62);
        Ok(())
    }
}
