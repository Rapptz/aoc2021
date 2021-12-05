use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.trim().split_once(',').ok_or(anyhow!("no delim"))?;
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

struct Graph {
    data: HashMap<(i32, i32), i32>,
}

impl Graph {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn add(&mut self, p1: Point, p2: Point, diagonals: bool) {
        let (min, max) = if p1 <= p2 { (p1, p2) } else { (p2, p1) };
        if min.x == max.x {
            for y in min.y..=max.y {
                self.data
                    .entry((min.x, y))
                    .and_modify(|i| *i += 1)
                    .or_insert(1);
            }
        } else if min.y == max.y {
            for x in min.x..=max.x {
                self.data
                    .entry((x, min.y))
                    .and_modify(|i| *i += 1)
                    .or_insert(1);
            }
        } else if diagonals {
            let inc = if min.y >= max.y { -1 } else { 1 };
            let mut y = min.y;
            for x in min.x..=max.x {
                self.data.entry((x, y)).and_modify(|i| *i += 1).or_insert(1);
                y += inc;
            }
        }
    }

    fn count(&self) -> usize {
        self.data.values().filter(|&v| *v >= 2).count()
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut graph = Graph::new();
    for line in input.lines() {
        let (p1, p2) = line.split_once(" -> ").ok_or(anyhow!("bad input"))?;
        graph.add(p1.parse()?, p2.parse()?, false);
    }
    Ok(graph.count())
}

fn part2(input: &str) -> Result<usize> {
    let mut graph = Graph::new();
    for line in input.lines() {
        let (p1, p2) = line.split_once(" -> ").ok_or(anyhow!("bad input"))?;
        graph.add(p1.parse()?, p2.parse()?, true);
    }
    Ok(graph.count())
}

fn main() -> Result<()> {
    let input = include_str!("day05_input.txt");
    println!("{:?}", part1(input)?);
    println!("{:?}", part2(input)?);
    Ok(())
}
