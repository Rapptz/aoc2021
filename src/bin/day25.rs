use std::fmt::Write;

use anyhow::Result;
use aoc2021::{FromCell, Grid};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Cell {
    East,
    South,
    Empty,
}

impl FromCell for Cell {
    fn from_cell(c: char) -> Option<Self> {
        match c {
            '>' => Some(Cell::East),
            'v' => Some(Cell::South),
            '.' => Some(Cell::Empty),
            _ => None,
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::East => f.write_char('>'),
            Cell::South => f.write_char('v'),
            Cell::Empty => f.write_char('.'),
        }
    }
}

struct Map {
    grid: Grid<Cell>,
}

impl Map {
    fn from_input(s: &str) -> Self {
        Self {
            grid: Grid::from_cells(s).unwrap(),
        }
    }

    fn step(&mut self) -> usize {
        let mut moves = Vec::new();
        for (x, y) in self.grid.coordinates() {
            match self.grid[(x, y)] {
                Cell::East => {
                    let dx = (x + 1) % self.grid.width();
                    if self.grid.get(dx, y) == Some(&Cell::Empty) {
                        moves.push((x, y));
                    }
                }
                _ => continue,
            }
        }

        let total = moves.len();
        for (x, y) in moves.drain(..) {
            let dx = (x + 1) % self.grid.width();
            self.grid[(x, y)] = Cell::Empty;
            self.grid[(dx, y)] = Cell::East;
        }

        for (x, y) in self.grid.coordinates() {
            match self.grid[(x, y)] {
                Cell::South => {
                    let dy = (y + 1) % self.grid.height();
                    if self.grid.get(x, dy) == Some(&Cell::Empty) {
                        moves.push((x, y));
                    }
                }
                _ => continue,
            }
        }

        let total = total + moves.len();
        for (x, y) in moves {
            let dy = (y + 1) % self.grid.height();
            self.grid[(x, y)] = Cell::Empty;
            self.grid[(x, dy)] = Cell::South;
        }

        total
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut map = Map::from_input(input);
    let mut steps = 0;
    loop {
        steps += 1;
        if map.step() == 0 {
            break;
        }
    }

    Ok(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 58);
    }

    #[test]
    fn test_part1_display() {
        let mut map = Map::from_input(INPUT);
        println!("{}", &map.grid);
        map.step();
        println!("{}", &map.grid);
    }
}

fn main() -> Result<()> {
    let input = include_str!("day25_input.txt").trim_end();
    println!("{:?}", part1(input)?);
    Ok(())
}
