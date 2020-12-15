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
    let mut next_number = 0;
    let mut last_seen: Vec<History> = vec![History::new(); turns];
    for &n in initial {
        last_seen[n].add(turn);
        next_number = last_seen[n].diff();
        last_number = n;
        turn += 1;
    }
    while turn <= turns {
        next_number = last_seen[last_number].diff();
        last_seen[next_number].add(turn);
        last_number = next_number;
        turn += 1;
    }
    next_number
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
        Ok(())
    }

    // #[test]
    // fn test_part2() -> Result<()> {
    //     assert_eq!(part2("0,3,6")?, 175594);
    //     Ok(())
    // }
}
