use anyhow::{anyhow, bail, Error, Result};
use aoc2020::coor::Coor;
use aoc2020::dispatch;
use std::convert::TryFrom;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn as_coor(&self) -> Coor {
        use Direction::*;

        match self {
            North => Coor::new(0, 1),
            South => Coor::new(0, -1),
            East => Coor::new(1, 0),
            West => Coor::new(-1, 0),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        use Direction::*;

        Ok(match c {
            'N' => North,
            'S' => South,
            'E' => East,
            'W' => West,
            _ => bail!("invalid direction `{}`", c),
        })
    }
}

#[derive(Debug, PartialEq)]
enum Turn {
    Left,
    Right,
}

impl Turn {
    fn apply(&self, coor: Coor) -> Coor {
        match self {
            Turn::Right => Coor::new(coor.y, -coor.x),
            Turn::Left => Coor::new(-coor.y, coor.x),
        }
    }
}

impl TryFrom<char> for Turn {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        use Turn::*;

        Ok(match c {
            'R' => Right,
            'L' => Left,
            _ => bail!("invalid turn `{}`", c),
        })
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Direction(Direction, i64),
    Turn(Turn, i64),
    Forward(i64),
}

impl TryFrom<&str> for Instruction {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        let val: i64 = s[1..].parse()?;
        let c = s.chars().next().ok_or(anyhow!("empty string"))?;
        if let Ok(direction) = Direction::try_from(c) {
            return Ok(Instruction::Direction(direction, val));
        }
        if let Ok(turn) = Turn::try_from(c) {
            return Ok(Instruction::Turn(turn, val));
        }
        if c == 'F' {
            return Ok(Instruction::Forward(val));
        }
        bail!("invalid direction `{}`", s);
    }
}

fn parse(input: &str) -> Result<Vec<Instruction>> {
    input
        .split('\n')
        .map(|s| Instruction::try_from(s))
        .collect()
}

fn turn(coor: Coor, direction: Turn, amount: i64) -> Coor {
    // (0 -1)  (x)  =  (-y)
    // (1  0)  (y)  =  ( x)
    if amount == 0 {
        coor
    } else {
        turn(direction.apply(coor), direction, amount - 90)
    }
}

fn part1(input: &str) -> Result<i64> {
    use Instruction::*;

    let instructions = parse(input)?;
    let mut direction = Coor::new(1, 0);
    let mut position = Coor::new(0, 0);
    for instruction in instructions {
        match instruction {
            Direction(to, v) => position += to.as_coor() * v,
            Turn(to, v) => direction = turn(direction, to, v),
            Forward(v) => position += direction * v,
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
            Direction(to, v) => waypoint += to.as_coor() * v,
            Turn(to, v) => waypoint = turn(waypoint, to, v),
            Forward(v) => position += waypoint * v,
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
