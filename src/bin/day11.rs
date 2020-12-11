use anyhow::{bail, Error, Result};
use aoc2020::coor::Coor;
use aoc2020::dispatch;
use std::convert::TryFrom;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        Ok(match c {
            '.' => Tile::Floor,
            'L' => Tile::Empty,
            _ => bail!("Invalid tile `{}`", c),
        })
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        Map { tiles }
    }

    fn from_str(input: &str) -> Result<Self> {
        parse(input).map(|t| Self::new(t))
    }

    fn get(&self, coor: &Coor) -> Option<Tile> {
        if coor.y < 0 || coor.x < 0 {
            return None;
        }
        let row = self.tiles.get(coor.y as usize)?;
        row.get(coor.x as usize).map(|&t| t)
    }

    fn occupied_neighbours(&self, coor: &Coor) -> usize {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .map(|&(dx, dy)| Coor::new(dx, dy) + *coor)
        .filter_map(|c| self.get(&c))
        .filter(|&t| t == Tile::Occupied)
        .count()
    }

    fn occupied_los_neighbours(&self, coor: &Coor) -> usize {
        let mut seen = 0;
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for direction in directions.iter().map(|&(dx, dy)| Coor::new(dx, dy)) {
            let mut pos = *coor + direction;
            while let Some(tile) = self.get(&pos) {
                if tile == Tile::Empty {
                    break;
                }
                if tile == Tile::Occupied {
                    seen += 1;
                    break;
                }
                pos += direction;
            }
        }
        seen
    }

    fn next_tiles(&self) -> (bool, Vec<Vec<Tile>>) {
        let mut changed = false;
        let tiles = self
            .tiles
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, tile)| {
                        (
                            tile,
                            self.occupied_neighbours(&Coor::new(x as i64, y as i64)),
                        )
                    })
                    .map(|(&tile, n)| match (tile, n) {
                        (Tile::Empty, 0) => {
                            changed = true;
                            Tile::Occupied
                        }
                        (Tile::Occupied, m) if m >= 4 => {
                            changed = true;
                            Tile::Empty
                        }
                        (tile, _) => tile,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        (changed, tiles)
    }

    fn next_tiles_los(&self) -> (bool, Vec<Vec<Tile>>) {
        let mut changed = false;
        let tiles = self
            .tiles
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, tile)| {
                        (
                            tile,
                            self.occupied_los_neighbours(&Coor::new(x as i64, y as i64)),
                        )
                    })
                    .map(|(&tile, n)| match (tile, n) {
                        (Tile::Empty, 0) => {
                            changed = true;
                            Tile::Occupied
                        }
                        (Tile::Occupied, m) if m >= 5 => {
                            changed = true;
                            Tile::Empty
                        }
                        (tile, _) => tile,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        (changed, tiles)
    }

    fn run(&mut self) -> usize {
        loop {
            let (changed, tiles) = self.next_tiles();
            if !changed {
                break;
            }
            self.tiles = tiles;
        }
        self.tiles
            .iter()
            .map(|r| r.iter().filter(|&t| *t == Tile::Occupied).count())
            .sum()
    }

    fn run_los(&mut self) -> usize {
        loop {
            let (changed, tiles) = self.next_tiles_los();
            if !changed {
                break;
            }
            self.tiles = tiles;
        }
        self.tiles
            .iter()
            .map(|r| r.iter().filter(|&t| *t == Tile::Occupied).count())
            .sum()
    }
}

fn parse(input: &str) -> Result<Vec<Vec<Tile>>> {
    input
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|c| Tile::try_from(c))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()
}

fn part1(input: &str) -> Result<usize> {
    let mut map = Map::from_str(input)?;
    Ok(map.run())
}

fn part2(input: &str) -> Result<usize> {
    let mut map = Map::from_str(input)?;
    Ok(map.run_los())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 37);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 26);
        Ok(())
    }
}
