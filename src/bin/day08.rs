use anyhow::{anyhow, bail, Error, Result};
use aoc2020::dispatch;
use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl TryFrom<&str> for Instruction {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        let n: i64 = s[4..].parse()?;
        Ok(match &s[..3] {
            "acc" => Instruction::Acc(n),
            "jmp" => Instruction::Jmp(n),
            "nop" => Instruction::Nop(n),
            _ => bail!("Invalid instruction `{}`", s),
        })
    }
}

fn main() -> Result<()> {
    dispatch(part1, part2)
}

type Program = Vec<Instruction>;

fn parse(input: &str) -> Result<Program> {
    input
        .split("\n")
        .map(|l| Instruction::try_from(l))
        .collect::<Result<_>>()
}

#[derive(Debug, PartialEq)]
enum RunResult {
    Loops(i64),
    Terminates(i64),
}

fn run(program: &Program) -> Result<RunResult> {
    let mut pc: i64 = 0;
    let mut acc = 0;
    let mut seen = HashSet::new();
    while !seen.contains(&pc) {
        if pc == program.len() as i64 {
            return Ok(RunResult::Terminates(acc));
        }
        seen.insert(pc);
        if pc < 0 {
            bail!("segfault");
        }
        let instruction: &Instruction = program.get(pc as usize).ok_or(anyhow!("segfault"))?;
        match instruction {
            Instruction::Acc(n) => {
                acc += n;
                pc += 1
            }
            Instruction::Jmp(n) => pc += n,
            Instruction::Nop(_) => pc += 1,
        }
    }
    Ok(RunResult::Loops(acc))
}

fn part1(input: &str) -> Result<i64> {
    let program = parse(input)?;
    if let RunResult::Loops(acc) = run(&program)? {
        Ok(acc)
    } else {
        bail!("Unexpectedly terminates")
    }
}

fn swap(instruction: &Instruction) -> Instruction {
    match instruction {
        Instruction::Nop(n) => Instruction::Jmp(*n),
        Instruction::Jmp(n) => Instruction::Nop(*n),
        Instruction::Acc(n) => Instruction::Acc(*n),
    }
}

fn part2(input: &str) -> Result<i64> {
    let mut program = parse(input)?;
    for index in 0..program.len() {
        program[index] = swap(&program[index]);
        if let RunResult::Terminates(val) = run(&program)? {
            return Ok(val);
        }
        program[index] = swap(&program[index]);
    }
    bail!("no terminating solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 5);
        Ok(())
    }

    #[test]
    fn test_loops() -> Result<()> {
        let program = parse(INPUT)?;
        assert_eq!(run(&program)?, RunResult::Loops(5));
        Ok(())
    }

    #[test]
    fn test_terminates() -> Result<()> {
        let program = parse(
            "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6",
        )?;
        assert_eq!(run(&program)?, RunResult::Terminates(8));
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 8);
        Ok(())
    }
}
