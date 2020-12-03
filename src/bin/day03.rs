use anyhow::{bail, Error, Result};
use aoc2020::coor::Coor;
use aoc2020::dispatch;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Open,
    Tree,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        Ok(match c {
            '.' => Tile::Open,
            '#' => Tile::Tree,
            _ => bail!("Invalid tile `{}`", c),
        })
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn new(input: &str) -> Result<Self> {
        let mut tiles = vec![];
        for row in input.split('\n') {
            let mut row_tiles = vec![];
            for c in row.chars() {
                row_tiles.push(Tile::try_from(c)?);
            }
            tiles.push(row_tiles);
        }
        Ok(Map { tiles })
    }

    fn get_at(&self, pos: Coor) -> Option<Tile> {
        let row = self.tiles.get(pos.y as usize)?;
        Some(*row.get(pos.x as usize % row.len()).expect("Rows wrap"))
    }

    fn traverse(&self, step: Coor) -> usize {
        let mut trees = 0;
        let mut pos = Coor::new(0, 0);
        while let Some(tile) = self.get_at(pos) {
            pos += step;
            if matches!(tile, Tile::Tree) {
                trees += 1;
            }
        }
        trees
    }
}

const STEPS: [Coor; 5] = [
    Coor::new(1, 1),
    Coor::new(3, 1),
    Coor::new(5, 1),
    Coor::new(7, 1),
    Coor::new(1, 2),
];

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<usize> {
    let map = Map::new(input)?;
    let step = STEPS[1];
    Ok(map.traverse(step))
}

fn part2(input: &str) -> Result<usize> {
    let map = Map::new(input)?;
    Ok(STEPS.iter().map(|s| map.traverse(*s)).product())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 7);
        Ok(())
    }

    #[test]
    fn test_traverse() -> Result<()> {
        let map = Map::new(INPUT)?;
        for (step, trees) in STEPS.iter().zip(vec![2, 7, 3, 4, 2].iter()) {
            assert_eq!(map.traverse(*step), *trees);
        }
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 336);
        Ok(())
    }
}
