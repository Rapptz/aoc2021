use std::collections::{HashMap, HashSet};

use anyhow::Result;

const SIZE: isize = 10;

struct Grid {
    data: HashMap<(isize, isize), u8>,
    flashes: usize,
}

impl Grid {
    fn from_input(s: &str) -> Grid {
        let data: HashMap<_, _> = s
            .lines()
            .enumerate()
            .flat_map(|(y, s)| {
                s.bytes()
                    .enumerate()
                    .map(move |(x, v)| ((x as isize, y as isize), v - b'0'))
            })
            .collect();
        Self { data, flashes: 0 }
    }

    #[rustfmt::skip]
    fn neighbouring_indexes(&self, x: isize, y: isize) -> [(isize, isize); 8] {
        // 0 1 2
        // 3 X 4
        // 5 6 7
        [
            (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
            (x - 1, y), /*          */  (x + 1, y),
            (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
        ]
    }

    fn process_flashes(&mut self, flashed: &mut HashSet<(isize, isize)>, x: isize, y: isize) {
        if let Some(value) = self.data.get_mut(&(x, y)) {
            if flashed.contains(&(x, y)) {
                return;
            }

            *value += 1;
            if *value > 9 {
                *value = 0;
                flashed.insert((x, y));
                for (x, y) in self.neighbouring_indexes(x, y) {
                    self.process_flashes(flashed, x, y)
                }
            }
        }
    }

    fn step(&mut self) -> bool {
        let mut flashed = HashSet::new();
        for x in 0..SIZE {
            for y in 0..SIZE {
                self.process_flashes(&mut flashed, x, y);
            }
        }
        self.flashes += flashed.len();
        flashed.len() == (SIZE * SIZE) as usize
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut grid = Grid::from_input(input);
    for _ in 0..100 {
        grid.step();
    }
    Ok(grid.flashes)
}

fn part2(input: &str) -> Result<usize> {
    let mut grid = Grid::from_input(input);
    let mut step = 1;
    loop {
        if grid.step() {
            return Ok(step);
        }
        step += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526";
        assert_eq!(part1(input).unwrap(), 1656)
    }

    #[test]
    fn test_part2() {
        let input = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526";
        assert_eq!(part2(input).unwrap(), 195)
    }
}

fn main() -> Result<()> {
    let input = include_str!("day11_input.txt").trim_end();
    println!("{:?}", part1(input)?);
    println!("{:?}", part2(input)?);
    Ok(())
}
