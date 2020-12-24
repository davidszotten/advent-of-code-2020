use anyhow::{bail, Error, Result};
use aoc2020::dispatch;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::ops::Add;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, PartialEq)]
enum Direction {
    E,
    NE,
    NW,
    W,
    SE,
    SW,
}

impl Direction {
    fn as_coor(&self) -> Coor {
        use Direction::*;
        match self {
            E => Coor { x: 1, y: -1, z: 0 },
            W => Coor { x: -1, y: 1, z: 0 },
            NW => Coor { x: 0, y: 1, z: -1 },
            SE => Coor { x: 0, y: -1, z: 1 },
            NE => Coor { x: 1, y: 0, z: -1 },
            SW => Coor { x: -1, y: 0, z: 1 },
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self> {
        let match_one = |t: &str| {
            Ok(match &t[..1] {
                "e" => Direction::E,
                "w" => Direction::W,
                _ => bail!("Invalid string(1) `{}`", &t[..1]),
            })
        };
        if s.len() <= 1 {
            match_one(s)
        } else {
            Ok(match &s[..2] {
                "ne" => Direction::NE,
                "nw" => Direction::NW,
                "se" => Direction::SE,
                "sw" => Direction::SW,
                t => match_one(&t[..1])?,
            })
        }
    }
}

fn parse(input: &str) -> Result<Vec<Direction>> {
    let mut res = vec![];
    let mut start = 0;
    while start < input.len() {
        let direction = Direction::try_from(&input[start..])?;
        start += match direction {
            Direction::E | Direction::W => 1,
            _ => 2,
        };
        res.push(direction);
    }
    Ok(res)
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
struct Coor {
    x: i64,
    y: i64,
    z: i64,
}

struct NeighbourIterator<'a> {
    coor: &'a Coor,
    pos: usize,
}

impl<'a> NeighbourIterator<'a> {
    fn new(coor: &'a Coor) -> Self {
        Self { coor, pos: 0 }
    }
}

impl<'a> Iterator for NeighbourIterator<'a> {
    type Item = Coor;
    fn next(&mut self) -> Option<Self::Item> {
        let positions = [
            (1, -1, 0),
            (-1, 1, 0),
            (1, 0, -1),
            (-1, 0, 1),
            (0, 1, -1),
            (0, -1, 1),
        ];
        let pos = self.pos;
        self.pos += 1;
        positions.get(pos).map(|(x, y, z)| {
            Coor {
                x: *x,
                y: *y,
                z: *z,
            } + *self.coor
        })
    }
}

impl Coor {
    fn neighbours(&self) -> NeighbourIterator {
        NeighbourIterator::new(self)
    }
}

impl Add for Coor {
    type Output = Coor;

    fn add(self, other: Coor) -> Coor {
        Coor {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

type Tile = Vec<Direction>;

fn get_floor(input: &str) -> Result<HashSet<Coor>> {
    let mut colors = HashMap::new();
    let tiles = input
        .split('\n')
        .map(|s| parse(s))
        .collect::<Result<Vec<Tile>>>()?;
    for tile in tiles {
        let mut coor = Coor { x: 0, y: 0, z: 0 };
        for direction in tile {
            coor = coor + direction.as_coor();
        }
        let val = colors.entry(coor).or_insert(false);
        *val = !*val;
    }
    Ok(colors
        .iter()
        .filter(|(_, v)| **v)
        .map(|(c, _)| *c)
        .collect::<HashSet<Coor>>())
}

fn part1(input: &str) -> Result<usize> {
    let colors = get_floor(input)?;
    Ok(colors.len())
}

fn flip(black_tiles: HashSet<Coor>) -> HashSet<Coor> {
    let mut next = HashSet::new();

    let black_neighbours = |tile: &Coor| {
        tile.neighbours()
            .map(|n| black_tiles.contains(&n))
            .filter(|t| *t)
            .count()
    };

    for tile in &black_tiles {
        for neighbour in tile.neighbours() {
            if black_tiles.contains(&neighbour) {
                continue;
            }

            let bc = black_neighbours(&neighbour);
            if bc == 2 {
                next.insert(neighbour.clone());
            }
        }
    }

    for tile in &black_tiles {
        let bc = black_neighbours(tile);
        if bc == 0 || bc > 2 {
        } else {
            next.insert(tile.clone());
        }
    }

    next
}

fn part2(input: &str) -> Result<usize> {
    let mut colors = get_floor(input)?;
    for _ in 0..100 {
        colors = flip(colors);
    }
    Ok(colors.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 10);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 2208);
        Ok(())
    }

    #[test]
    fn test_part1a() -> Result<()> {
        assert_eq!(part1("nwwswee")?, 1);
        Ok(())
    }

    #[test]
    fn test_try_from() -> Result<()> {
        use Direction::*;
        assert_eq!(Direction::try_from("e")?, E);
        assert_eq!(Direction::try_from("se")?, SE);
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<()> {
        use Direction::*;
        assert_eq!(parse("esenee")?, vec![E, SE, NE, E]);
        Ok(())
    }
}
