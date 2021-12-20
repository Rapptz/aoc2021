use anyhow::Result;
use itertools::Itertools;
use std::{collections::HashSet, fmt::Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Rect {
    x: isize,
    y: isize,
    width: isize,
    height: isize,
}

impl Rect {
    #[inline]
    fn contains(&self, x: isize, y: isize) -> bool {
        x >= self.x && x <= self.width && y >= self.y && y <= self.height
    }
}

#[derive(Debug)]
struct Image {
    algorithm: Vec<bool>,
    input: HashSet<(isize, isize)>,
    bounds: Rect,
    default_pixel: bool,
}

const ADJACENT: [(isize, isize); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

impl Image {
    fn from_input(s: &str) -> Option<Self> {
        let (algorithm, s) = s.split_once("\n\n")?;
        let algorithm: Vec<bool> = algorithm.chars().map(|ch| ch == '#').collect();
        let input: HashSet<_> = s
            .lines()
            .enumerate()
            .flat_map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .filter_map(move |(x, v)| (v == '#').then(|| (x as isize, y as isize)))
            })
            .collect();

        let (x, width) = input.iter().map(|p| p.0).minmax().into_option()?;
        let (y, height) = input.iter().map(|p| p.1).minmax().into_option()?;

        Some(Self {
            // mfw gotcha
            default_pixel: false,
            algorithm,
            input,
            bounds: Rect {
                x,
                y,
                width,
                height,
            },
        })
    }

    fn get(&self, x: isize, y: isize) -> usize {
        if !self.bounds.contains(x, y) {
            self.default_pixel as usize
        } else {
            self.input.contains(&(x, y)) as usize
        }
    }

    fn binary_at(&self, x: isize, y: isize) -> usize {
        ADJACENT
            .iter()
            .enumerate()
            .fold(0, |acc, (index, (dx, dy))| {
                acc | (self.get(x + dx, y + dy) << (8 - index))
            })
    }

    fn step(&mut self) {
        // I wish I could avoid this reallocation
        let mut new_image = HashSet::with_capacity(
            self.input.len() + self.bounds.width as usize * 2 + self.bounds.height as usize * 2,
        );
        for x in self.bounds.x - 1..=self.bounds.width + 1 {
            for y in self.bounds.y - 1..=self.bounds.height + 1 {
                let index = self.binary_at(x, y);
                if self.algorithm[index] {
                    new_image.insert((x, y));
                }
            }
        }
        if self.default_pixel {
            self.default_pixel = self.algorithm.last().copied().unwrap_or(false);
        } else {
            self.default_pixel = self.algorithm.first().copied().unwrap_or(true);
        }

        self.input = new_image;
        self.bounds.x -= 1;
        self.bounds.width += 1;
        self.bounds.y -= 1;
        self.bounds.height += 1;
    }

    fn pixels(&self) -> usize {
        self.input.len()
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let default = if self.default_pixel { '#' } else { '.' };
        for y in self.bounds.y - 1..=self.bounds.height + 1 {
            for x in self.bounds.x - 1..=self.bounds.width + 1 {
                if self.input.contains(&(x, y)) {
                    f.write_char('#')?;
                } else {
                    f.write_char(default)?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> Option<usize> {
    let mut image = Image::from_input(input)?;
    for _ in 0..2 {
        image.step();
    }
    Some(image.pixels())
}

fn part2(input: &str) -> Option<usize> {
    let mut image = Image::from_input(input)?;
    for _ in 0..50 {
        image.step();
    }
    Some(image.pixels())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 3351);
    }
}

fn main() -> Result<()> {
    let input = include_str!("day20_input.txt").trim_end();
    println!("{:?}", part1(input).unwrap());
    println!("{:?}", part2(input).unwrap());
    Ok(())
}
