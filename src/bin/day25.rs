use anyhow::Result;
use aoc2020::dispatch;
use aoc2020::mod_arith::mod_mul;

const REM: usize = 20201227;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn transform(subj: usize, loop_size: usize) -> usize {
    let mut val = 1;
    for _ in 0..loop_size {
        val = mod_mul(val, subj as i64, REM as i64);
    }
    val as usize
}

fn find_loop(key: usize) -> usize {
    let mut val = 1;
    let mut loop_size = 1;
    loop {
        val = mod_mul(val, 7, REM as i64);
        if val == key as i64 {
            break loop_size;
        }
        loop_size += 1;
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut lines = input.split('\n');
    let n1: usize = lines.next().expect("should have 2 numbers").parse()?;
    let n2: usize = lines.next().expect("should have 2 numbers").parse()?;
    let loop_size = find_loop(n1);
    Ok(transform(n2, loop_size))
}

fn part2(_input: &str) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1("5764801\n17807724")?, 14897079);
        Ok(())
    }
}
