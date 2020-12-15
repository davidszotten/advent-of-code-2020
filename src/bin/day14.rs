use anyhow::{anyhow, bail, Error, Result};
use aoc2020::dispatch;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Default)]
struct Mask {
    zeros: usize,
    ones: usize,
    floating: usize,
}

impl Mask {
    fn apply(&self, value: usize) -> usize {
        (value & !self.zeros) | self.ones
    }

    fn apply_float(&self, addr: usize) -> Vec<usize> {
        let addr = addr | self.ones;
        let mut res = vec![addr];
        let mut bit = 1;
        while bit <= self.floating {
            if bit & self.floating == bit {
                let mut new = vec![];
                for addr in res {
                    new.push(!bit & addr);
                    new.push(bit | addr);
                }
                res = new;
            }
            bit <<= 1;
        }
        res
    }
}

impl TryFrom<&str> for Mask {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        let mut mask = Mask::default();
        for (idx, c) in s.chars().rev().enumerate() {
            *match c {
                '0' => &mut mask.zeros,
                '1' => &mut mask.ones,
                'X' => &mut mask.floating,
                _ => bail!("invalid char `{}`", c),
            } += 1 << idx;
        }
        Ok(mask)
    }
}

enum Instruction {
    Mask(Mask),
    MemSet(usize, usize),
}

impl TryFrom<&str> for Instruction {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
                // mask = 0X0X1110X1010X1X10010X0011010X100110
                // mem[40190] = 23031023
                Regex::new(r"mask = (?P<mask>[01X]{36})|mem\[(?P<mem>\d+)\] = (?P<val>\d+)")
                    .expect("invalid regex");
        }
        let caps = RE
            .captures(s)
            .ok_or(anyhow!("regex mismatch for `{}`", s))?;

        Ok(if let Some(raw_mask) = caps.name("mask") {
            let mask = Mask::try_from(raw_mask.as_str())?;
            Instruction::Mask(mask)
        } else {
            let raw_mem = caps.name("mem").ok_or(anyhow!("mem not found"))?;
            let raw_val = caps.name("val").ok_or(anyhow!("val not found"))?;
            Instruction::MemSet(raw_mem.as_str().parse()?, raw_val.as_str().parse()?)
        })
    }
}

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<usize> {
    let mut mask = Mask::default();
    let mut mem = HashMap::new();
    for instruction in input.split('\n').map(Instruction::try_from) {
        match instruction? {
            Instruction::Mask(new_mask) => mask = new_mask,
            Instruction::MemSet(addr, value) => {
                mem.insert(addr, mask.apply(value));
            }
        }
    }
    Ok(mem.values().sum())
}

fn part2(input: &str) -> Result<usize> {
    let mut mask = Mask::default();
    let mut mem = HashMap::new();
    for instruction in input.split('\n').map(Instruction::try_from) {
        match instruction? {
            Instruction::Mask(new_mask) => mask = new_mask,
            Instruction::MemSet(addr, value) => {
                for addr in mask.apply_float(addr) {
                    mem.insert(addr, value);
                }
            }
        }
    }
    Ok(mem.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_parse() -> Result<()> {
        assert_eq!(
            Mask::try_from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")?,
            Mask {
                zeros: 2,
                ones: 64,
                floating: 68719476669
            }
        );
        Ok(())
    }

    #[test]
    fn test_mask_parse2() -> Result<()> {
        assert_eq!(
            Mask::try_from("000000000000000000000000000000X1001X")?,
            Mask {
                zeros: 68719476684,
                ones: 18,
                floating: 33
            }
        );
        Ok(())
    }

    const INPUT: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_mask_apply() {
        assert_eq!(
            Mask {
                zeros: 2,
                ones: 64,
                floating: 0
            }
            .apply(11),
            73
        );
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 165);
        Ok(())
    }

    #[test]
    fn test_mask_apply_float() -> Result<()> {
        let mut addresses = Mask::try_from("000000000000000000000000000000X1001X")?.apply_float(42);
        addresses.sort();
        assert_eq!(addresses, vec![26, 27, 58, 59]);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            part2(
                "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            )?,
            208
        );
        Ok(())
    }
}
