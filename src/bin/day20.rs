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

fn as_char(el: &&bool) -> char {
    match **el {
        false => '.',
        true => '#',
    }
}

fn edge_to_num<'a, I>(edge: I, debug: bool) -> usize
where
    I: Iterator<Item = &'a bool>,
{
    let foo: Vec<_> = edge.collect();
    if debug {
        println!("edge: {}", foo.iter().map(as_char).collect::<String>());
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

    fn image_row(&self, edge: DirectedEdge, row: usize) -> Vec<&bool> {
        match (edge.edge, edge.reversed) {
            (Edge::Up, false) => self.data[row].iter().collect(),
            (Edge::Down, false) => self.data[self.size - 1 - row].iter().collect(),
            (Edge::Left, false) => self.data.iter().map(|r| &r[row]).collect(),
            (Edge::Right, false) => self.data.iter().map(|r| &r[self.size - 1 - row]).collect(),

            (Edge::Up, true) => self.data[row].iter().rev().collect(),
            (Edge::Down, true) => self.data[self.size - 1 - row].iter().rev().collect(),
            (Edge::Left, true) => self.data.iter().rev().map(|r| &r[row]).collect(),
            (Edge::Right, true) => self
                .data
                .iter()
                .rev()
                .map(|r| &r[self.size - 1 - row])
                .collect(),
        }
    }

    fn as_base_edge(&self, orientation: Orientation, edge: Edge) -> DirectedEdge {
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
        base[index]
    }

    fn edge_hash(&self, orientation: Orientation, edge: Edge, debug: bool) -> usize {
        let base = self.as_base_edge(orientation, edge);
        self.base_edge_hash(base, debug)
    }

    fn _print(&self) {
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

fn find_grid(input: &str) -> Result<Vec<(Tile, Orientation)>> {
    let tiles = parse(input)?;
    let size = (tiles.len() as f32).sqrt() as usize;
    let map = Map { size };
    let mut queue = VecDeque::new();
    for tile_idx in 0..tiles.len() {
        for &mirrored in &[false, true] {
            for &top in &[Edge::Up, Edge::Right, Edge::Down, Edge::Left] {
                queue.push_back(vec![(tile_idx, Orientation { top, mirrored })]);
            }
        }
    }

    while let Some(grid) = queue.pop_front() {
        if grid.len() == tiles.len() && grid[0].1.mirrored {
            return Ok(grid
                .iter()
                .map(|(tile_idx, orientation)| (tiles[*tile_idx].clone(), *orientation))
                .collect::<Vec<_>>());
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
                    let next_orientation = Orientation { top, mirrored };
                    let mut all_neighbours_ok = true;
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
                        }
                    }
                    if debug {
                        dbg!(all_neighbours_ok);
                    }
                    if !all_neighbours_ok {
                        continue;
                    }

                    let mut next_grid = grid.clone();
                    next_grid.push((next_tile_idx, next_orientation));
                    queue.push_back(next_grid)
                }
            }
        }
    }
    bail!("failed to assemble grid");
}

fn part1(input: &str) -> Result<usize> {
    let grid = find_grid(input)?;
    let size = (grid.len() as f32).sqrt() as usize;

    Ok([0, size - 1, grid.len() - size, grid.len() - 1]
        .iter()
        .map(|&idx| grid[idx].0.number)
        .product::<usize>())
}

fn rotate(im: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let rows = im.len();
    let cols = im[0].len();

    let mut new = vec![];
    for col in 0..cols {
        let mut new_row = vec![];
        for row in 0..rows {
            new_row.push(im[row][col]);
        }
        new.push(new_row);
    }
    new
}

fn flip(im: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    im.into_iter()
        .map(|row| row.into_iter().rev().collect())
        .collect()
}

fn part2(input: &str) -> Result<usize> {
    let grid = find_grid(input)?;
    let size = (grid.len() as f32).sqrt() as usize;
    let tile_size = grid[0].0.size;
    let map = Map { size };

    let mut image: Vec<Vec<&bool>> = vec![];
    for large_row in 0..size {
        for tile_row in 1..(tile_size - 1) {
            let mut row = vec![];
            for large_column in 0..size {
                let grid_index = map
                    .to_index(Coor::new(large_column as _, large_row as _))
                    .expect("should be in range");
                let (tile, orientation) = &grid[grid_index];
                let edge = tile.as_base_edge(*orientation, Edge::Up);
                let image_row = tile.image_row(edge, tile_row);
                row.extend_from_slice(&image_row[1..image_row.len() - 1]);
            }
            image.push(row);
        }
    }
    let monster = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ]
    .iter()
    .map(|l| {
        l.chars()
            .map(|c| {
                Ok(match c {
                    ' ' => false,
                    '#' => true,
                    _ => bail!("invalid tile `{}`", c),
                })
            })
            .collect::<Result<Vec<_>>>()
    })
    .collect::<Result<Vec<_>>>()?;

    for &do_flip in &[false, true] {
        for rotations in 0..4 {
            let mut sea = image.iter().map(|r| r.clone()).collect::<Vec<_>>();
            let mut rotation_monsters = 0;
            let mut rotated_monster = monster.clone();
            if do_flip {
                rotated_monster = flip(rotated_monster);
            }
            for _ in 0..rotations {
                rotated_monster = rotate(rotated_monster);
            }

            for start_row in 0..(&image.len() - rotated_monster.len()) {
                for start_col in 0..(image[0].len() - rotated_monster[0].len()) {
                    let mut found_monster = true;
                    for (row_offset, monster_row) in rotated_monster.iter().enumerate() {
                        for (col_offset, &monster_val) in monster_row.iter().enumerate() {
                            let image_row = &image[start_row + row_offset];
                            if monster_val && !*image_row[start_col + col_offset] {
                                if !do_flip && rotations == 0 && start_row == 3 && start_col == 2 {}
                                found_monster = false;
                            }
                        }
                    }
                    if found_monster {
                        rotation_monsters += 1;
                        for (row_offset, monster_row) in rotated_monster.iter().enumerate() {
                            for (col_offset, monster_val) in monster_row.iter().enumerate() {
                                if *monster_val {
                                    sea[start_row + row_offset][start_col + col_offset] = &false;
                                }
                            }
                        }
                    }
                }
            }
            if rotation_monsters > 0 {
                return Ok(sea
                    .iter()
                    .map(|r| r.iter().filter(|b| ***b).count())
                    .sum::<usize>());
            }
        }
    }

    bail!("didn't find any sea monsters");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<()> {
        let input = include_str!("day20.sample");
        let tiles = parse(input)?;
        Ok(())
    }

    #[test]
    fn test_edge_to_num() {
        assert_eq!(edge_to_num([false, true, false, false].iter(), false), 4)
    }

    #[test]
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
        tile._print();
        let orientation = Orientation {
            top: Edge::Left,
            mirrored: false,
        };
        tile.edge_hash(orientation, Edge::Up, true);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let input = include_str!("day20.sample");
        assert_eq!(part1(input)?, 20899048083289);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = include_str!("day20.sample");
        assert_eq!(part2(input)?, 273);
        Ok(())
    }
}
