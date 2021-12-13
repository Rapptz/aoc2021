use std::{
    collections::HashSet,
    fmt::{Debug, Display, Write},
    str::FromStr,
};

use anyhow::{bail, Result};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: u16,
    y: u16,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", &self.x, &self.y)
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(anyhow::anyhow!("bad input"))?;
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Fold {
    X(u16),
    Y(u16),
}

impl Fold {
    fn apply<'a, T>(&self, points: T) -> HashSet<Point>
    where
        T: Iterator<Item = &'a Point>,
    {
        match self {
            Fold::X(x) => points
                .map(|f| {
                    if f.x < *x {
                        *f
                    } else {
                        Point {
                            x: 2 * x - f.x,
                            y: f.y,
                        }
                    }
                })
                .collect(),
            Fold::Y(y) => points
                .map(|f| {
                    if f.y < *y {
                        *f
                    } else {
                        Point {
                            x: f.x,
                            y: 2 * y - f.y,
                        }
                    }
                })
                .collect(),
        }
    }
}

impl FromStr for Fold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, rest) = s.split_at(11);
        let (coord, value) = rest.split_once('=').ok_or(anyhow::anyhow!("bad input"))?;
        match coord {
            "x" => Ok(Self::X(value.parse()?)),
            "y" => Ok(Self::Y(value.parse()?)),
            _ => bail!("bad input"),
        }
    }
}

#[derive(Debug)]
struct Paper {
    coords: HashSet<Point>,
    folds: Vec<Fold>,
}

impl Paper {
    fn new(s: &str) -> Self {
        let (coords, folds) = s.split_once("\n\n").unwrap();
        Self {
            coords: coords.lines().filter_map(|s| s.parse().ok()).collect(),
            folds: folds.lines().filter_map(|s| s.parse().ok()).collect(),
        }
    }

    fn fold(&mut self, all: bool) {
        let folds = if all {
            self.folds.iter()
        } else {
            self.folds[0..1].iter()
        };
        for fold in folds {
            self.coords = fold.apply(self.coords.iter());
        }
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // ugly
        let max_x = self
            .coords
            .iter()
            .max_by_key(|f| f.x)
            .map(|p| p.x)
            .unwrap_or_default();
        let min_x = self
            .coords
            .iter()
            .min_by_key(|f| f.x)
            .map(|p| p.x)
            .unwrap_or_default();

        let max_y = self
            .coords
            .iter()
            .max_by_key(|f| f.y)
            .map(|p| p.y)
            .unwrap_or_default();
        let min_y = self
            .coords
            .iter()
            .min_by_key(|f| f.y)
            .map(|p| p.y)
            .unwrap_or_default();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.coords.contains(&Point { x, y }) {
                    f.write_char('X')?;
                } else {
                    f.write_char(' ')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut paper = Paper::new(input);
    paper.fold(false);
    Ok(paper.coords.len())
}

fn part2(input: &str) -> Result<Paper> {
    let mut paper = Paper::new(input);
    paper.fold(true);
    Ok(paper)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_part1() {
        let paper = Paper::new(INPUT);
        println!("{}", &paper);
        assert_eq!(part1(INPUT).unwrap(), 17);
    }
}

fn main() -> Result<()> {
    let input = include_str!("day13_input.txt").trim_end();
    println!("{:?}", part1(input)?);
    println!("{}", part2(input)?);
    Ok(())
}
