use anyhow::{anyhow, bail, Result};
use aoc2020::dispatch;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() -> Result<()> {
    dispatch(part1, part2)
}

type Cups = VecDeque<usize>;

fn round(mut cups: Cups) -> Cups {
    let len = cups.len();
    // current: cups[0]
    let current = cups.pop_front().expect("always have cups");
    cups.push_back(current);
    // cups.rotate_left(1);
    let mut pickup = vec![];
    // current now last, pick up first 3
    for _ in 0..3 {
        pickup.push(cups.pop_front().unwrap());
    }
    let sub_wrap = |n| {
        let s = (n + len - 1) % len;
        if s == 0 {
            len
        } else {
            s
        }
    };
    let mut destination = sub_wrap(current);
    while destination == pickup[0] || destination == pickup[1] || destination == pickup[2] {
        destination = sub_wrap(destination);
    }
    // dbg!(&cups);
    // dbg!(destination);
    let index = cups
        .iter()
        .enumerate()
        .filter(|(_, &val)| val == destination)
        .map(|(idx, _)| idx)
        .next()
        .expect("should find destination");
    // dbg!(&cups, destination, index);
    for _ in 0..3 {
        cups.insert(index + 1, pickup.pop().expect("have 3"));
    }
    let current_index = cups
        .iter()
        .enumerate()
        .filter(|(_, &val)| val == current)
        .map(|(idx, _)| idx)
        .next()
        .expect("should find destination");
    // dbg!(current_index, &cups);
    cups.rotate_left(current_index + 1);
    cups
}

type CupsH = HashMap<usize, usize>;

fn round_h(current: usize, mut cups: CupsH) -> (usize, CupsH) {
    let len = cups.len();

    let next = |current| *cups.get(&current).unwrap();

    let sub_wrap = |n| {
        let s = (n + len - 1) % len;
        if s == 0 {
            len
        } else {
            s
        }
    };
    let mut destination = sub_wrap(current);
    // dbg!(destination);

    let pickup1 = next(current);
    // dbg!(pickup1);
    // let mut after_pickup = first_pickup;
    // let mut last_pickup = after_pickup;

    let pickup2 = next(pickup1);
    let pickup3 = next(pickup2);
    let after_pickup = next(pickup3);

    while destination == pickup1 || destination == pickup2 || destination == pickup3 {
        destination = sub_wrap(destination);
    }

    // dbg!(after_pickup);
    // dbg!((current, after_pickup));
    cups.insert(current, after_pickup);
    let after_destination = *cups.get(&destination).unwrap();
    // dbg!((destination, pickup1));
    cups.insert(destination, pickup1);
    //// cups.insert(after_pickup, after_destination);
    // dbg!((pickup3, after_destination));
    cups.insert(pickup3, after_destination);

    let next = *cups.get(&current).unwrap();
    (next, cups)
}

fn run(mut cups: Cups, moves: usize) -> String {
    for _mv in 0..moves {
        // println!("Move {}\n{:?}\n", mv + 1, &game.cups);
        cups = round(cups);
    }

    let one_index = cups
        .iter()
        .enumerate()
        .filter(|(_, &val)| val == 1)
        .map(|(idx, _)| idx)
        .next()
        .expect("should find destination");
    cups.rotate_left(one_index);

    let mut res = "".to_string();
    for cup in cups.iter().skip(1) {
        res = format!("{}{}", res, cup);
    }
    res
}

#[allow(dead_code)]
fn run_h(mut cur: usize, mut cups: CupsH, moves: usize) -> String {
    for _mv in 0..moves {
        // println!("Move {}\n{:?}\n", mv + 1, &game.cups);
        println!("Move {}", _mv + 1);
        print_h(cur, &cups);
        println!("");
        let next = round_h(cur, cups);
        cur = next.0;
        cups = next.1;
    }
    print_h(cur, &cups);
    println!("");

    let mut n = 1;
    let mut res = "".to_string();
    while *cups.get(&n).unwrap() != 1 {
        n = *cups.get(&n).unwrap();
        res = format!("{}{}", res, n);
    }

    res
}

#[allow(dead_code)]
fn print_h(current: usize, cups: &CupsH) {
    let mut n = *cups.get(&current).unwrap();
    print!("{}, ", current);
    while n != current {
        print!("{}, ", n);
        n = *cups.get(&n).unwrap();
    }
    println!("");
}

fn parse(input: &str) -> Result<Cups> {
    let cups = input
        .chars()
        .map(|c| {
            c.to_string()
                .parse()
                .map_err(|e| anyhow!("parse failure: {}", e))
        })
        .collect::<Result<VecDeque<usize>>>()?;
    Ok(cups)
}

fn parse_h(input: &str) -> Result<(usize, usize, CupsH)> {
    let cups = input
        .chars()
        .map(|c| {
            c.to_string()
                .parse()
                .map_err(|e| anyhow!("parse failure: {}", e))
        })
        .collect::<Result<Vec<usize>>>()?;
    let mut cups_h = HashMap::new();
    for win in cups.windows(2) {
        cups_h.insert(win[0], win[1]);
    }
    let last = cups[cups.len() - 1];
    cups_h.insert(last, cups[0]);
    Ok((cups[0], last, cups_h))
}

fn part1(input: &str) -> Result<String> {
    let cups = parse(input)?;
    // println!("Move {}\n{:?}\n", 0 + 1, &cups);
    let res = run(cups, 100);
    // println!("Move {}\n{:?}\n", "end", &cups);
    Ok(res)

    // 52863971 too high
}

fn _part2a(input: &str) -> Result<usize> {
    let mut cups = parse(input)?;
    // for n in cups.len()..1_000_000 {
    for n in (cups.len() + 1)..=20 {
        cups.push_back(n);
    }
    let mut seen = HashSet::new();
    // let mut seen_cycles = HashSet::new();
    let mut seen_cycles_at = HashMap::new();
    for _mv in 0..100 {
        // let pos = index_of(&cups.make_contiguous(), 3).unwrap();
        // cups.rotate_left(pos);
        // println!("Move {}\n{:?}\n", _mv + 1, &cups);
        // cups.rotate_right(pos);
        if seen.contains(&cups) {
            bail!("repeat");
        }
        seen.insert(cups.clone());
        let mut prev = cups.clone();
        cups = round(cups);
        let cycles = as_disjoint_cycles(prev.make_contiguous(), cups.make_contiguous());
        // for cycle in &cycles {
        // println!("{:?}", cycle);
        // }
        // println!("");
        // if seen_cycles.contains(&cycles) {
        // bail!("repeated cycle at {}", _mv);
        // }
        // seen_cycles.insert(cycles);
        seen_cycles_at.entry(cycles).or_insert(vec![]).push(_mv);
    }
    for moves in seen_cycles_at.values() {
        println!("{:?}", moves);
    }
    Ok(0)
}

fn part2(input: &str) -> Result<usize> {
    let (mut cur, mut last, mut cups) = parse_h(input)?;
    for n in (cups.len() + 1)..=1_000_000 {
        // for n in (cups.len() + 1)..=20 {
        cups.insert(last, n);
        last = n;
    }
    cups.insert(last, cur);

    // dbg!(&cups);
    for _mv in 0..10_000_000 {
        let next = round_h(cur, cups);
        cur = next.0;
        cups = next.1;
    }
    let a = *cups.get(&1).unwrap();
    let b = *cups.get(&a).unwrap();
    dbg!(a, b);
    Ok(a * b)
}

#[allow(dead_code)]

fn index_of(vec: &[usize], val: usize) -> Option<usize> {
    vec.iter()
        .enumerate()
        .filter(|(_, &v)| v == val)
        .map(|(i, _)| i)
        .next()
}

#[allow(dead_code)]
fn as_disjoint_cycles(a: &[usize], b: &[usize]) -> Vec<Vec<usize>> {
    let mut res = vec![];
    let mut seen = HashSet::new();

    for (index, &val) in a.iter().enumerate() {
        if seen.contains(&val) {
            continue;
        }
        seen.insert(val);
        let mut cycle = vec![];
        cycle.push(index + 1);
        let start = index;
        let mut next_index = index_of(b, val).unwrap();
        while next_index != start {
            cycle.push(next_index + 1);
            let val = a[next_index];
            seen.insert(val);
            next_index = index_of(b, val).unwrap();
        }
        res.push(cycle);
    }

    res
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
    fn test_run() -> Result<()> {
        let cups = parse("389125467")?;
        let res = run(cups, 10);
        assert_eq!(res, "92658374");
        Ok(())
    }

    #[test]
    fn test_round_h() -> Result<()> {
        let (curr, _, cups) = parse_h("389125467")?;
        let (next, res) = round_h(curr, cups);
        print_h(next, &res);
        assert!(false);
        Ok(())
        // assert_eq!(res, "92658374");
    }

    #[test]
    fn test_run_h() -> Result<()> {
        let (cur, _, cups) = parse_h("389125467")?;
        let res = run_h(cur, cups, 10);
        assert_eq!(res, "92658374");
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2("389125467")?, 149245887792);
        Ok(())
    }
}
