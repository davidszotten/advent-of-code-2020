use anyhow::{anyhow, bail, Result};
use aoc2020::coor::Coor;
use aoc2020::dispatch;
use std::collections::{HashSet, VecDeque};

fn main() -> Result<()> {
    dispatch(part1, part2)
}

struct Map {
    size: usize,
}

impl Map {
    fn to_index(&self, coor: Coor) -> Option<usize> {
        let size = self.size as i64;
        if coor.x < 0 || coor.x >= size || coor.y < 0 || coor.y >= size {
            return None;
        }
        Some((coor.y * size + coor.x) as usize)
    }

    fn from_index(&self, index: usize) -> Coor {
        let y = index / self.size;
        let x = index % self.size;
        Coor::new(x as i64, y as i64)
    }

    fn neighbours(&self, index: usize, debug: bool) -> Vec<(usize, Edge)> {
        if debug {
            dbg!(self.size);
        }
        let mut res = vec![];
        let coor = self.from_index(index);
        for (offset, edge) in &[
            (Coor::new(-1, 0), Edge::Right),
            (Coor::new(1, 0), Edge::Left),
            (Coor::new(0, -1), Edge::Down),
            (Coor::new(0, 1), Edge::Up),
        ] {
            if let Some(index) = self.to_index(*offset + coor) {
                res.push((index, *edge));
            }
        }
        res
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Edge {
    Up,
    Down,
    Left,
    Right,
}

impl Edge {
    fn facing(&self) -> Edge {
        use Edge::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DirectedEdge {
    edge: Edge,
    reversed: bool,
}

impl DirectedEdge {
    fn new(edge: Edge, reversed: bool) -> Self {
        DirectedEdge { edge, reversed }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Orientation {
    mirrored: bool,
    top: Edge,
}

#[derive(Debug, Clone)]
struct Tile {
    number: usize,
    size: usize,
    data: Vec<Vec<bool>>,
}

fn edge_to_num<'a, I>(edge: I, debug: bool) -> usize
where
    I: Iterator<Item = &'a bool>,
{
    let foo: Vec<_> = edge.collect();
    if debug {
        println!(
            "edge: {}",
            foo.iter()
                .map(|e| match e {
                    false => '.',
                    true => '#',
                })
                .collect::<String>()
        );
    }
    let edge = foo.iter();
    edge.fold(0, |acc, &el| (acc << 1) + *el as usize)
}

impl Tile {
    fn from_str(input: &str) -> Result<Self> {
        let mut lines = input.split('\n').peekable();
        let title = lines.next().ok_or(anyhow!("title missing"))?;
        let number = title["Tile ".len()..title.len() - 1].parse()?;
        let size = lines.peek().ok_or(anyhow!("grid missing"))?.len();
        let data = lines
            .filter(|l| l.len() > 0)
            .map(|l| {
                l.chars()
                    .map(|c| {
                        Ok(match c {
                            '.' => false,
                            '#' => true,
                            _ => bail!("invalid tile `{}`", c),
                        })
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Tile { number, size, data })
    }

    fn base_edge_hash(&self, edge: DirectedEdge, debug: bool) -> usize {
        match (edge.edge, edge.reversed) {
            (Edge::Up, false) => edge_to_num(self.data[0].iter(), debug),
            (Edge::Down, false) => edge_to_num(self.data[self.size - 1].iter(), debug),
            (Edge::Left, false) => edge_to_num(self.data.iter().map(|r| &r[0]), debug),
            (Edge::Right, false) => edge_to_num(self.data.iter().map(|r| &r[self.size - 1]), debug),

            (Edge::Up, true) => edge_to_num(self.data[0].iter().rev(), debug),
            (Edge::Down, true) => edge_to_num(self.data[self.size - 1].iter().rev(), debug),
            (Edge::Left, true) => edge_to_num(self.data.iter().rev().map(|r| &r[0]), debug),
            (Edge::Right, true) => {
                edge_to_num(self.data.iter().rev().map(|r| &r[self.size - 1]), debug)
            }
        }
    }

    // fn edge_numbers(&self) -> Vec<usize> {
    //     let mut res = vec![];
    //     res.push(edge_to_num(self.data[0].iter()));
    //     res.push(edge_to_num(self.data[0].iter().rev()));

    //     res.push(edge_to_num(self.data[self.size - 1].iter()));
    //     res.push(edge_to_num(self.data[self.size - 1].iter().rev()));

    //     res.push(edge_to_num(self.data.iter().map(|r| &r[0])));
    //     res.push(edge_to_num(self.data.iter().rev().map(|r| &r[0])));

    //     res.push(edge_to_num(self.data.iter().map(|r| &r[self.size - 1])));
    //     res.push(edge_to_num(
    //         self.data.iter().rev().map(|r| &r[self.size - 1]),
    //     ));
    //     res
    // }

    fn edge_hash(&self, orientation: Orientation, edge: Edge, debug: bool) -> usize {
        let mut base = if !orientation.mirrored {
            /*
              U1 U2
              L1 R1
              L2 R2
              D1 D2
            */
            [
                DirectedEdge::new(Edge::Up, false),
                DirectedEdge::new(Edge::Right, false),
                DirectedEdge::new(Edge::Down, false),
                DirectedEdge::new(Edge::Left, false),
            ]
        } else {
            /*
              U2 U1
              R1 L1
              R2 L2
              D2 D1
            */
            [
                DirectedEdge::new(Edge::Up, true),
                DirectedEdge::new(Edge::Left, false),
                DirectedEdge::new(Edge::Down, true),
                DirectedEdge::new(Edge::Right, false),
            ]
        };
        while base[0].edge != orientation.top {
            if debug {
                println!("rotate");
            }
            base.rotate_right(1);
            base[0].reversed = !base[0].reversed;
            base[2].reversed = !base[2].reversed;
        }
        let index = match edge {
            Edge::Up => 0,
            Edge::Right => 1,
            Edge::Down => 2,
            Edge::Left => 3,
        };
        if debug {
            dbg!(base, index);
        }
        self.base_edge_hash(base[index], debug)
    }

    fn print(&self) {
        for row in &self.data {
            for c in row {
                print!(
                    "{}",
                    match c {
                        false => '.',
                        true => '#',
                    }
                );
            }
            print!("\n");
        }
    }
}

fn parse(input: &str) -> Result<Vec<Tile>> {
    input
        .split("\n\n")
        .map(|s| Tile::from_str(s))
        .collect::<Result<Vec<_>>>()
}

fn part1(input: &str) -> Result<usize> {
    let tiles = parse(input)?;
    let size = (tiles.len() as f32).sqrt() as usize;
    let map = Map { size };
    let mut queue = VecDeque::new();
    // let mut seen = HashSet::new();
    for tile_idx in 0..tiles.len() {
        for &mirrored in &[false, true] {
            for &top in &[Edge::Up, Edge::Right, Edge::Down, Edge::Left] {
                queue.push_back(vec![(tile_idx, Orientation { top, mirrored })]);
            }
        }
    }

    let mut _count = 0;
    let mut max_len = 0;
    while let Some(grid) = queue.pop_front() {
        _count += 1;
        if grid.len() > max_len {
            max_len = dbg!(grid.len());
        }
        // let grid_debug = grid
        // .iter()
        // .map(|idx| tiles[idx.0].number)
        // .collect::<Vec<_>>();

        // if grid_debug == vec![1951, 2311, 3079, 2729, 1427]
        //, 2473]
        // {
        // dbg!("ok");
        // }
        // if _count > 1 {
        // break;
        // }
        if grid.len() == tiles.len() {
            // dbg!(&grid_debug);
            // dbg!(&[0, size - 1, grid.len() - size, grid.len() - 1]);
            return Ok(
                // println!(
                // "{}",
                [0, size - 1, grid.len() - size, grid.len() - 1]
                    .iter()
                    .map(|&idx| grid[idx].0)
                    .map(|tile_idx| tiles[tile_idx].number)
                    .product::<usize>(),
            );
            // continue;
        }
        let used = grid.iter().map(|t| t.0).collect::<HashSet<_>>();
        for next_tile_idx in 0..tiles.len() {
            if used.contains(&next_tile_idx) {
                continue;
            }
            let next_tile = &tiles[next_tile_idx];
            for &mirrored in &[false, true] {
                for &top in &[Edge::Up, Edge::Right, Edge::Down, Edge::Left] {
                    let debug = false;
                    // let debug = grid.len() == 5
                    // && tiles[grid[0].0].number == 1951
                    // && grid[0].1.mirrored
                    // && grid[0].1.top == Edge::Down
                    // && tiles[grid[1].0].number == 2311
                    // && grid[1].1.mirrored
                    // && grid[1].1.top == Edge::Down
                    // && tiles[grid[2].0].number == 3079
                    // && !grid[2].1.mirrored
                    // && grid[2].1.top == Edge::Up
                    // && tiles[grid[3].0].number == 2729
                    // && grid[3].1.mirrored
                    // && grid[3].1.top == Edge::Down
                    // && tiles[grid[4].0].number == 1427
                    // && grid[4].1.mirrored
                    // && grid[4].1.top == Edge::Down
                    // && next_tile.number == 2473
                    // && mirrored && (top == Edge::Right || top == Edge::Left);
                    let next_orientation = Orientation { top, mirrored };
                    let mut all_neighbours_ok = true;
                    if debug {
                        dbg!(next_tile_idx);
                        // dbg!(map.neighbours(grid.len(), debug));
                    }
                    for (grid_neighbour_idx, neighbour_edge) in map.neighbours(grid.len(), false) {
                        if let Some((tiles_neighbour_idx, neighbour_orientation)) =
                            grid.get(grid_neighbour_idx)
                        {
                            let neighbour = &tiles[*tiles_neighbour_idx];
                            if debug {
                                dbg!(
                                    grid_neighbour_idx,
                                    tiles_neighbour_idx,
                                    &neighbour.number,
                                    neighbour_orientation,
                                    neighbour_edge
                                );
                                // neighbour.print();
                            }
                            let neighbour_edge_hash =
                                neighbour.edge_hash(*neighbour_orientation, neighbour_edge, debug);
                            if debug {
                                dbg!(&next_tile.number);
                                dbg!(next_orientation);
                                dbg!(neighbour_edge.facing());
                            }
                            let next_edge_hash = next_tile.edge_hash(
                                next_orientation,
                                neighbour_edge.facing(),
                                debug,
                            );
                            if neighbour_edge_hash != next_edge_hash {
                                all_neighbours_ok = false;
                            }
                        } else {
                            // dbg!("no neighbour", neighbour_idx);
                        }
                    }
                    if debug {
                        dbg!(all_neighbours_ok);
                    }
                    if !all_neighbours_ok {
                        continue;
                    }

                    // dbg!("match");
                    let mut next_grid = grid.clone();
                    next_grid.push((next_tile_idx, next_orientation));
                    // if seen.contains(&next_grid) {
                    // continue;
                    // }
                    // seen.insert(next_grid.clone());
                    queue.push_back(next_grid)
                }
            }
        }
    }
    Ok(0)
}

fn part2(_input: &str) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<()> {
        let input = include_str!("day20.sample");
        let tiles = parse(input)?;
        dbg!(&tiles[0]);
        Ok(())
    }

    #[test]
    fn test_edge_to_num() {
        assert_eq!(edge_to_num([false, true, false, false].iter(), false), 4)
    }

    #[test]
    fn test_part1() -> Result<()> {
        let input = include_str!("day20.sample");
        assert_eq!(part1(input)?, 20899048083289);
        Ok(())
    }

    // #[test]
    fn test_edge_hash() -> Result<()> {
        let tile = Tile::from_str(
            "Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.",
        )?;
        tile.print();
        let orientation = Orientation {
            top: Edge::Left,
            mirrored: false,
        };
        // tile.edge_hash(orientation, Edge::Up, true);
        // tile.edge_hash(orientation, Edge::Right, true);
        // tile.edge_hash(orientation, Edge::Down, true);
        tile.edge_hash(orientation, Edge::Up, true);
        assert!(false);
        Ok(())
    }
}
