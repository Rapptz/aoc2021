use std::collections::{HashMap, HashSet};

use anyhow::Result;

struct Cave {
    data: HashMap<(usize, usize), u8>,
    width: usize,
    height: usize,
}

impl Cave {
    fn new(s: &str) -> Self {
        let data: HashMap<_, _> = s
            .lines()
            .enumerate()
            .flat_map(|(y, s)| s.bytes().enumerate().map(move |(x, v)| ((x, y), v - b'0')))
            .collect();

        let height = s.lines().count();
        let width = data
            .keys()
            .max_by_key(|x| x.0)
            .map(|x| x.0 + 1)
            .unwrap_or_default();

        Self {
            data,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.data.get(&(x, y)).copied()
    }

    fn neighbours(&self, x: usize, y: usize) -> [Option<u8>; 4] {
        let mut result = [None; 4];
        if y != 0 {
            result[0] = self.get(x, y - 1);
        }
        if y + 1 < self.height {
            result[1] = self.get(x, y + 1);
        }
        if x != 0 {
            result[2] = self.get(x - 1, y);
        }
        if x + 1 < self.width {
            result[3] = self.get(x + 1, y);
        }
        result
    }

    fn neighbouring_points(&self, x: usize, y: usize) -> [Option<(usize, usize)>; 4] {
        let mut result = [None; 4];
        if y != 0 {
            result[0] = Some((x, y - 1));
        }
        if y + 1 < self.height {
            result[1] = Some((x, y + 1));
        }
        if x != 0 {
            result[2] = Some((x - 1, y));
        }
        if x + 1 < self.width {
            result[3] = Some((x + 1, y));
        }
        result
    }

    fn low_point(&self, x: usize, y: usize) -> Option<u8> {
        let current = self.get(x, y)?;
        let neighbours = self.neighbours(x, y);
        let low = neighbours.iter().all(|x| match x {
            None => true,
            Some(x) => current < *x,
        });

        if low {
            Some(current)
        } else {
            None
        }
    }
}

fn part1(input: &str) -> Result<usize> {
    let caves = Cave::new(input);
    let mut points = vec![];
    for ((x, y), _) in caves.data.iter() {
        if let Some(pt) = caves.low_point(*x, *y) {
            points.push(pt);
        }
    }

    Ok(points.iter().map(|x| *x as usize + 1).sum())
}

fn part2(input: &str) -> Result<usize> {
    let caves = Cave::new(input);
    let mut basins = vec![];
    let mut points = vec![];
    for ((x, y), _) in caves.data.iter() {
        if let Some(_) = caves.low_point(*x, *y) {
            points.push((*x, *y));
        }
    }

    for point in points {
        let mut visited = HashSet::new();
        let mut queue = vec![point];
        while let Some(pt) = queue.pop() {
            for neighbour in caves.neighbouring_points(pt.0, pt.1) {
                match neighbour {
                    None => continue,
                    Some(p) => {
                        match caves.get(p.0, p.1) {
                            None => continue,
                            Some(x) => {
                                if x == 9 || visited.get(&p).is_some() {
                                    continue;
                                }
                                visited.insert(p);
                                queue.push(p);
                            }
                        }
                    }
                }
            }
        }
        basins.push(visited.len());
    }

    basins.sort_by(|a, b| b.cmp(a));
    Ok(basins[0] * basins[1] * basins[2])
}

fn main() -> Result<()> {
    let input = include_str!("day09_input.txt").trim_end();
    println!("{:?}", part1(input)?);
    println!("{:?}", part2(input)?);
    Ok(())
}
