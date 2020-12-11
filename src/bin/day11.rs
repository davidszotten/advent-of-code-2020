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

#[derive(Debug, PartialEq, Clone, Copy)]
enum NeighbourType {
    Adjacent,
    LoS,
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
}

impl Map {
    fn from_str(input: &str) -> Result<Self> {
        let width = input.find('\n').unwrap_or(input.len());
        let tiles = input
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| Tile::try_from(c))
            .collect::<Result<Vec<_>>>()?;
        Ok(Map { tiles, width })
    }

    fn height(&self) -> usize {
        self.tiles.len() / self.width
    }

    fn idx(&self, coor: &Coor) -> Option<usize> {
        if coor.y < 0
            || coor.y >= (self.height() as i64)
            || coor.x < 0
            || coor.x >= (self.width as i64)
        {
            return None;
        }
        Some((coor.y as usize) * self.width + (coor.x as usize))
    }

    fn coor(&self, idx: usize) -> Coor {
        Coor::new((idx % self.width) as i64, (idx / self.width) as i64)
    }

    fn get(&self, coor: &Coor) -> Option<Tile> {
        let idx = self.idx(coor)?;
        self.tiles.get(idx).map(|&t| t)
    }

    fn occupied_neighbours(&self, coor: &Coor, nt: NeighbourType) -> usize {
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
        .map(|&(dx, dy)| Coor::new(dx, dy))
        .map(|direction| {
            let mut occupied = 0;
            let mut pos = *coor + direction;
            while let Some(tile) = self.get(&pos) {
                if tile == Tile::Empty {
                    break;
                }
                if tile == Tile::Occupied {
                    occupied += 1;
                    break;
                }
                match nt {
                    NeighbourType::Adjacent => break,
                    NeighbourType::LoS => pos += direction,
                }
            }
            occupied
        })
        .sum()
    }

    fn next_tiles(&self, nt: NeighbourType, min_neighbours: usize) -> Option<Vec<Tile>> {
        let mut changed = false;
        let tiles = self
            .tiles
            .iter()
            .enumerate()
            .map(|(idx, tile)| (tile, self.occupied_neighbours(&self.coor(idx), nt)))
            .map(|(&tile, n)| match (tile, n) {
                (Tile::Empty, 0) => {
                    changed = true;
                    Tile::Occupied
                }
                (Tile::Occupied, m) if m >= min_neighbours => {
                    changed = true;
                    Tile::Empty
                }
                (tile, _) => tile,
            })
            .collect::<Vec<_>>();

        if changed {
            Some(tiles)
        } else {
            None
        }
    }

    fn run(&mut self, nt: NeighbourType, min_neighbours: usize) -> usize {
        while let Some(tiles) = self.next_tiles(nt, min_neighbours) {
            self.tiles = tiles;
        }
        self.tiles.iter().filter(|&t| *t == Tile::Occupied).count()
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut map = Map::from_str(input)?;
    Ok(map.run(NeighbourType::Adjacent, 4))
}

fn part2(input: &str) -> Result<usize> {
    let mut map = Map::from_str(input)?;
    Ok(map.run(NeighbourType::LoS, 5))
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

    #[test]
    fn test_coor() -> Result<()> {
        let map = Map::from_str(
            "...
...",
        )?;
        for idx in 0..6 {
            assert_eq!(map.idx(&map.coor(idx)), Some(idx));
        }
        Ok(())
    }

    #[test]
    fn test_lookup() -> Result<()> {
        let map = Map::from_str(
            ".L.
...",
        )?;
        assert_eq!(map.get(&Coor::new(1, 0)), Some(Tile::Empty));
        assert_eq!(map.get(&Coor::new(3, 0)), None);
        assert_eq!(map.get(&Coor::new(0, 4)), None);
        Ok(())
    }
}
