use anyhow::{anyhow, Result};
use aoc2020::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn parse(input: &str) -> Result<Vec<usize>> {
    input
        .split(',')
        .map(|n| {
            n.parse::<usize>()
                .map_err(|e| anyhow!("parse failure: {}", e))
        })
        .collect::<Result<Vec<_>>>()
}

#[derive(Debug, Clone, Copy)]
struct History {
    first: Option<usize>,
    second: Option<usize>,
}

impl History {
    fn new() -> Self {
        History {
            first: None,
            second: None,
        }
    }
    fn add(&mut self, entry: usize) {
        self.second = self.first;
        self.first = Some(entry);
    }

    fn diff(&self) -> usize {
        match (self.first, self.second) {
            (Some(first), Some(second)) => first - second,
            _ => 0,
        }
    }
}

fn run(initial: &[usize], turns: usize) -> usize {
    let mut turn = 1;
    let mut last_number = 0;
    let mut last_seen: Vec<History> = vec![History::new(); turns];
    for &n in initial {
        last_number = n;
        last_seen[last_number].add(turn);
        turn += 1;
    }
    while turn <= turns {
        last_number = last_seen[last_number].diff();
        last_seen[last_number].add(turn);
        turn += 1;
    }
    last_number
}

fn part1(input: &str) -> Result<usize> {
    let initial = parse(input)?;
    Ok(run(&initial, 2020))
}

fn part2(input: &str) -> Result<usize> {
    let initial = parse(input)?;
    Ok(run(&initial, 30000000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1("0,3,6")?, 436);
        assert_eq!(part1("1,3,2")?, 1);
        assert_eq!(part1("2,1,3")?, 10);
        assert_eq!(part1("1,2,3")?, 27);
        assert_eq!(part1("2,3,1")?, 78);
        assert_eq!(part1("3,2,1")?, 438);
        assert_eq!(part1("3,1,2")?, 1836);
        Ok(())
    }

    // #[test]
    // fn test_part2() -> Result<()> {
    //     assert_eq!(part2("0,3,6")?, 175594);
    //     Ok(())
    // }
}
