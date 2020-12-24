use anyhow::{anyhow, Result};
use aoc2020::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

type Cups = Vec<usize>;

fn round(current: usize, mut cups: Cups) -> (usize, Cups) {
    let len = cups.len();

    let sub_wrap = |n| {
        let s = (n + len - 1) % len;
        if s == 0 {
            len
        } else {
            s
        }
    };
    let mut destination = sub_wrap(current);

    let pickup1 = cups[current - 1];

    let pickup2 = cups[pickup1 - 1];
    let pickup3 = cups[pickup2 - 1];
    let after_pickup = cups[pickup3 - 1];

    while destination == pickup1 || destination == pickup2 || destination == pickup3 {
        destination = sub_wrap(destination);
    }

    cups[current - 1] = after_pickup;
    let after_destination = cups[destination - 1];
    cups[destination - 1] = pickup1;
    cups[pickup3 - 1] = after_destination;

    let next_cur = cups[current - 1];
    (next_cur, cups)
}

fn run(mut cur: usize, mut cups: Cups, moves: usize) -> String {
    for _mv in 0..moves {
        println!("Move {}", _mv + 1);
        print(cur, &cups);
        println!("");
        let next = round(cur, cups);
        cur = next.0;
        cups = next.1;
    }
    print(cur, &cups);
    println!("");

    let mut n = 1;
    let mut res = "".to_string();
    while cups[n - 1] != 1 {
        n = cups[n - 1];
        res = format!("{}{}", res, n);
    }

    res
}

#[allow(dead_code)]
fn print(current: usize, cups: &Cups) {
    let mut n = cups[current - 1];
    print!("{}, ", current);
    while n != current {
        print!("{}, ", n);
        n = cups[n - 1];
    }
    println!("");
}

fn parse(input: &str) -> Result<(usize, usize, Cups)> {
    let cups_raw = input
        .chars()
        .map(|c| {
            c.to_string()
                .parse()
                .map_err(|e| anyhow!("parse failure: {}", e))
        })
        .collect::<Result<Vec<usize>>>()?;
    let mut cups = vec![0; cups_raw.len()];
    for win in cups_raw.windows(2) {
        cups[win[0] - 1] = win[1];
    }
    let first = cups_raw[0];
    let last = cups_raw[cups_raw.len() - 1];
    cups[last - 1] = first;
    Ok((first, last, cups))
}

fn part1(input: &str) -> Result<String> {
    let (first, _, cups) = parse(input)?;
    let res = run(first, cups, 100);
    Ok(res)
}

fn part2(input: &str) -> Result<usize> {
    let (mut cur, mut last, mut cups) = parse(input)?;
    for n in (cups.len() + 1)..=1_000_000 {
        cups.push(0);
        cups[last - 1] = n;
        last = n;
    }
    cups[last - 1] = cur;

    for _mv in 0..10_000_000 {
        let next = round(cur, cups);
        cur = next.0;
        cups = next.1;
    }
    let a = cups[1 - 1];
    let b = cups[a - 1];
    dbg!(a, b);
    Ok(a * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1("389125467")?, "67384529");
        Ok(())
    }

    #[test]
    fn test_round() -> Result<()> {
        let (curr, _, cups) = parse("389125467")?;
        let (next, res) = round(curr, cups);
        dbg!(&res);
        print(next, &res);
        Ok(())
        // assert_eq!(res, "92658374");
    }

    #[test]
    fn test_run() -> Result<()> {
        let (cur, _, cups) = parse("389125467")?;
        let res = run(cur, cups, 10);
        assert_eq!(res, "92658374");
        Ok(())
    }

    // too slow
    // #[test]
    fn _test_part2() -> Result<()> {
        assert_eq!(part2("389125467")?, 149245887792);
        Ok(())
    }
}
