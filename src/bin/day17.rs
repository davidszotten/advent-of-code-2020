use anyhow::Result;
use aoc2020::dispatch;
use std::collections::HashSet;
use std::ops::Add;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

trait Coor: Copy {
    fn from_index(idx: usize, size: usize) -> Self;

    fn apply_deltas(f: &mut dyn FnMut(Self));
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
struct Coor3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Coor3 {
    const fn new(x: i64, y: i64, z: i64) -> Self {
        Coor3 { x, y, z }
    }
}

impl Add for Coor3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coor3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl Coor for Coor3 {
    fn from_index(idx: usize, size: usize) -> Coor3 {
        let z = idx / size / size;
        let y = (idx - z * size * size) / size;
        let x = idx - z * size * size - y * size;
        Coor3::new(x as i64, y as i64, z as i64)
    }

    fn apply_deltas(f: &mut dyn FnMut(Coor3)) {
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    f(Coor3::new(x, y as i64, z as i64))
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
struct Coor4 {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Coor4 {
    const fn new(x: i64, y: i64, z: i64, w: i64) -> Self {
        Coor4 { x, y, z, w }
    }
}

impl Add for Coor4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coor4::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Coor for Coor4 {
    fn from_index(idx: usize, size: usize) -> Coor4 {
        let z = idx / size / size / size;
        let y = (idx - z * size * size * size) / size / size;
        let x = (idx - z * size * size * size - y * size * size) / size;
        let w = idx - z * size * size * size - y * size * size - x * size;
        Coor4::new(w as i64, x as i64, y as i64, z as i64)
    }

    fn apply_deltas(f: &mut dyn FnMut(Coor4)) {
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        f(Coor4::new(x, y as i64, z as i64, w as i64))
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Space<T>
where
    T: Coor,
{
    active: HashSet<T>,
    size: usize,
}

impl<T: Coor + Add<Output = T> + Default + Eq + std::hash::Hash> Space<T> {
    fn from_str(input: &str) -> Result<Self> {
        let size = input.find('\n').unwrap_or(input.len());
        let active = input
            .chars()
            .filter(|&c| c != '\n')
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(i, _)| T::from_index(i, size))
            .collect::<HashSet<_>>();
        Ok(Space { active, size })
    }

    fn occupied_neighbours(&self, coor: &T) -> usize {
        let mut count = 0;
        T::apply_deltas(&mut |delta| {
            if delta == T::default() {
                return;
            }
            if self.active.contains(&(*coor + delta)) {
                count += 1;
            }
        });
        count
    }

    fn next_tiles(&self) -> HashSet<T> {
        let mut to_consider = HashSet::new();
        for &coor in self.active.iter() {
            T::apply_deltas(&mut |delta| {
                to_consider.insert(coor + delta);
            });
        }
        let mut next = HashSet::new();
        for coor in to_consider {
            let is_active = self.active.contains(&coor);
            let new_state = match (is_active, self.occupied_neighbours(&coor)) {
                (true, n) if n == 2 || n == 3 => true,
                (false, n) if n == 3 => true,
                _ => false,
            };
            if new_state {
                next.insert(coor);
            }
        }
        next
    }

    fn run(&mut self, times: usize) -> usize {
        for _ in 0..times {
            self.active = self.next_tiles();
        }
        self.active.iter().count()
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut space: Space<Coor3> = Space::from_str(input)?;
    Ok(space.run(6))
}

fn part2(input: &str) -> Result<usize> {
    let mut space: Space<Coor4> = Space::from_str(input)?;
    Ok(space.run(6))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(
            part1(
                ".#.
..#
###"
            )?,
            112
        );
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            part2(
                ".#.
..#
###"
            )?,
            848
        );
        Ok(())
    }

    #[test]
    fn test_lookup() -> Result<()> {
        let space: Space<Coor3> = Space::from_str(
            "...
...
..#",
        )?;
        assert_eq!(space.active.len(), 1);
        assert_eq!(
            space.active,
            [Coor3::new(2, 2, 0)].iter().cloned().collect()
        );
        Ok(())
    }
}
