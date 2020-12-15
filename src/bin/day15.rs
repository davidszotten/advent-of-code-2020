use anyhow::{anyhow, Result};
use aoc2020::dispatch;
use std::collections::HashMap;

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

struct History {
    history: [usize; 2],
    len: usize,
}

impl History {
    fn new() -> Self {
        History {
            history: [0, 0],
            len: 0,
        }
    }
    fn add(&mut self, entry: usize) {
        self.history[1] = self.history[0];
        self.history[0] = entry;
    }
}

fn run(initial: &[usize], turns: usize) -> usize {
    let mut turn = 1;
    let mut last_number = 0;
    let mut next_number = 0;
    let mut last_seen = HashMap::new();
    for &n in initial {
        last_seen.entry(n).or_insert(vec![]).push(turn);
        let history = last_seen.get(&n).expect("just inserted");
        next_number = if history.len() > 1 {
            history[0] - history[1]
        } else {
            0
        };
        last_number = n;
        turn += 1;
    }
    while turn <= turns {
        // dbg!(turn, last_number);
        let history = last_seen.get(&last_number).expect("just inserted");
        next_number = if history.len() > 1 {
            // dbg!(&history);
            history[history.len() - 1] - history[history.len() - 2]
        } else {
            0
        };
        last_seen.entry(next_number).or_insert(vec![]).push(turn);
        last_number = next_number;
        // println!("say at {}: {}", turn, next_number);
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
