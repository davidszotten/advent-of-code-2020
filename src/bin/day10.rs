use anyhow::{anyhow, bail, Result};
use aoc2020::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn parse(input: &str) -> Result<Vec<i64>> {
    input
        .split('\n')
        .map(|r| r.parse::<i64>().map_err(|_| anyhow!("not a number")))
        .collect()
}

fn part1(input: &str) -> Result<i32> {
    let mut numbers = parse(input)?;
    numbers.push(0);
    numbers.sort();

    let mut ones = 0;
    let mut threes = 0;

    for win in numbers.windows(2) {
        match win[1] - win[0] {
            1 => ones += 1,
            2 => {}
            3 => threes += 1,
            _ => bail!("jump too large"),
        }
    }

    threes += 1;
    Ok(ones * threes)
}

fn part2(input: &str) -> Result<i64> {
    let mut numbers = parse(input)?;
    numbers.push(0);
    numbers.sort();

    let mut counts = vec![1];
    let mut count = 0;

    for (index, number) in numbers.iter().enumerate().skip(1) {
        count =
            // look at the previous 3 items. since numbers are monotonically increasing, only the
            // last 3 could possibly be within 3 of the current number
            (index.saturating_sub(3)..index)
            // filter out any that differ by more than 3
            .filter(|&prev_idx| number - numbers[prev_idx] <= 3)
            // sum their respective counts
            .map(|prev_idx| counts[prev_idx])
            .sum();
        counts.push(count);
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const INPUT2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT1)?, 7 * 5);
        Ok(())
    }

    #[test]
    fn test_part1b() -> Result<()> {
        assert_eq!(part1(INPUT2)?, 22 * 10);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT1)?, 8);
        Ok(())
    }

    #[test]
    fn test_part2b() -> Result<()> {
        assert_eq!(part2(INPUT2)?, 19208);
        Ok(())
    }
}
