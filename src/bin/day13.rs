use anyhow::{anyhow, Result};
use aoc2020::dispatch;
use aoc2020::mod_arith::mod_div;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn parse(input: &str) -> Result<(i64, Vec<i64>)> {
    let mut lines = input.split('\n');
    let earliest = lines.next().ok_or(anyhow!("too few lines"))?.parse()?;
    let bus_times = lines
        .next()
        .ok_or(anyhow!("too few lines"))?
        .split(',')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
    Ok((earliest, bus_times))
}

fn parse2(input: &str) -> Result<Vec<(i64, i64)>> {
    Ok(input
        .split('\n')
        .skip(1)
        .next()
        .ok_or(anyhow!("too few lines"))?
        .split(',')
        .enumerate()
        .filter_map(|(i, s)| s.parse::<i64>().map(|n| (-(i as i64), n)).ok())
        .collect())
}

fn part1(input: &str) -> Result<i64> {
    let (earliest, bus_times) = parse(input)?;
    let prod = bus_times
        .iter()
        .map(|t| (t, t - (earliest % t)))
        .min_by_key(|&(_, wait)| wait)
        .map(|(b, w)| b * w)
        .ok_or(anyhow!("no bus times"))?;
    Ok(prod)
}

// n == c1 mod n1
// n == c2 mod n2
// -> n == reduce.0 mod reduce.1
fn reduce(c1: i64, n1: i64, c2: i64, n2: i64) -> (i64, i64) {
    /*
    x = c1 mod n1
    x = c2 mod n2

    x = k n1 + c1
    k n1 + c1 = c2 mod n2
    k n1 = c2 - c1 mod n2
    k = (c2 - c1)/n1 mod n2
    k = n2 t + (c2-c1)/n1

    x = n1 n2 t +  ((c2-c2)/n1)*n1 + c1

    */
    let k = mod_div(c2 - c1, n1, n2);
    let c = k * n1 + c1;
    let n = n1 * n2;

    // add n (mod n) to keep c positive
    ((c + n) % n, n)
}

fn part2(input: &str) -> Result<i64> {
    let numbers = parse2(input)?;
    let red = numbers[1..]
        .iter()
        .fold(numbers[0], |acc, el| reduce(acc.0, acc.1, el.0, el.1));
    Ok(red.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 295);
        Ok(())
    }

    #[test]
    fn test_reduce() {
        assert_eq!(reduce(-2, 13, -3, 19), (206, 247));
        assert_eq!(reduce(-2, 13, 0, 17), (102, 221));
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2("\n17,x,13,19")?, 3417);
        assert_eq!(part2("\n67,7,59,61")?, 754018);
        assert_eq!(part2("\n67,x,7,59,61")?, 779210);
        assert_eq!(part2("\n67,7,x,59,61")?, 1261476);
        assert_eq!(part2("\n1789,37,47,1889")?, 1202161486);
        Ok(())
    }
}
