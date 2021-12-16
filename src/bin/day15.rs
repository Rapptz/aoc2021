use std::collections::BinaryHeap;

use anyhow::Result;
use aoc2021::{Direction, Grid};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NodeWithCost {
    x: usize,
    y: usize,
    cost: u16,
}

impl PartialOrd for NodeWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeWithCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn shortest_path(grid: &Grid<u8>) -> Option<u16> {
    let mut queue = BinaryHeap::new();
    queue.push(NodeWithCost {
        x: 0,
        y: 0,
        cost: 0,
    });

    let mut dist: Vec<_> = grid.items().map(|_| u16::MAX).collect();
    dist[0] = 0;

    while let Some(NodeWithCost { x, y, cost }) = queue.pop() {
        if (x, y) == (grid.width() - 1, grid.height() - 1) {
            return Some(cost);
        }

        for (dx, dy) in grid.neighbours(x, y, Direction::Cardinal) {
            let next = NodeWithCost {
                x: dx,
                y: dy,
                cost: cost + grid[(dx, dy)] as u16,
            };

            let idx = (dy * grid.width()) + dx;
            if next.cost < dist[idx] {
                queue.push(next);
                dist[idx] = next.cost;
            }
        }
    }
    None
}

fn part1(input: &str) -> Option<u16> {
    let grid = Grid::single_ascii_number(input);
    shortest_path(&grid)
}

struct GridRowExpansion<'a> {
    s: &'a str,
    current: u8,
    offset: u8,
    iter: std::str::Bytes<'a>,
}

impl<'a> GridRowExpansion<'a> {
    fn new(s: &'a str, offset: u8) -> GridRowExpansion<'a> {
        Self {
            current: 0,
            offset,
            s,
            iter: s.bytes(),
        }
    }
}

impl<'a> Iterator for GridRowExpansion<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => Some(((v - b'0' + self.offset + self.current - 1) % 9) + 1),
            None => {
                self.current += 1;
                if self.current >= 5 {
                    None
                } else {
                    self.iter = self.s.bytes();
                    self.next()
                }
            }
        }
    }
}

fn part2(input: &str) -> Option<u16> {
    let data: Vec<_> = (0..5)
        .flat_map(|offset| {
            input
                .lines()
                .flat_map(move |s| GridRowExpansion::new(s, offset))
        })
        .collect();

    let height = input.lines().count() * 5;
    let width = input
        .find('\n')
        .map(|x| x * 5)
        .unwrap_or_else(|| data.len() / height);

    let grid = Grid::with_data(data, width, height);
    shortest_path(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 315);
    }
}

fn main() -> Result<()> {
    let input = include_str!("day15_input.txt").trim_end();
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
    Ok(())
}
