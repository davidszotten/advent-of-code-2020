use anyhow::{anyhow, bail, Error, Result};
use aoc2020::coor::Coor;
use aoc2020::dispatch;
use std::convert::TryFrom;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, PartialEq)]
enum Instruction {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

impl TryFrom<&str> for Instruction {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        use Instruction::*;

        let val: i64 = s[1..].parse()?;
        Ok(match s.chars().next().ok_or(anyhow!("empty string"))? {
            'N' => North(val),
            'S' => South(val),
            'E' => East(val),
            'W' => West(val),
            'L' => Left(val),
            'R' => Right(val),
            'F' => Forward(val),
            _ => bail!("invalid direction `{}`", s),
        })
    }
}

fn parse(input: &str) -> Result<Vec<Instruction>> {
    input
        .split('\n')
        .map(|s| Instruction::try_from(s))
        .collect()
}

fn turn(direction: Coor, amount: i64) -> Coor {
    // (0 -1)  (x)  =  (-y)
    // (1  0)  (y)  =  ( x)
    if amount == 0 {
        direction
    } else if amount < 0 {
        turn(Coor::new(direction.y, -direction.x), amount + 90)
    } else {
        turn(Coor::new(-direction.y, direction.x), amount - 90)
    }
}

fn part1(input: &str) -> Result<i64> {
    use Instruction::*;

    let instructions = parse(input)?;
    let mut direction = Coor::new(1, 0);
    let mut position = Coor::new(0, 0);
    for instruction in instructions {
        match instruction {
            North(v) => position += Coor::new(0, v),
            South(v) => position += Coor::new(0, -v),
            East(v) => position += Coor::new(v, 0),
            West(v) => position += Coor::new(-v, 0),
            Left(v) => direction = turn(direction, v),
            Right(v) => direction = turn(direction, -v),
            Forward(v) => {
                for _ in 0..v {
                    position += direction
                }
            }
        }
    }
    Ok(position.x.abs() + position.y.abs())
}

fn part2(input: &str) -> Result<i64> {
    use Instruction::*;

    let instructions = parse(input)?;
    let mut waypoint = Coor::new(10, 1);
    let mut position = Coor::new(0, 0);
    for instruction in instructions {
        match instruction {
            North(v) => waypoint += Coor::new(0, v),
            South(v) => waypoint += Coor::new(0, -v),
            East(v) => waypoint += Coor::new(v, 0),
            West(v) => waypoint += Coor::new(-v, 0),
            Left(v) => waypoint = turn(waypoint, v),
            Right(v) => waypoint = turn(waypoint, -v),
            Forward(v) => {
                for _ in 0..v {
                    position += waypoint
                }
            }
        }
    }
    Ok(position.x.abs() + position.y.abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 25);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 286);
        Ok(())
    }
}
