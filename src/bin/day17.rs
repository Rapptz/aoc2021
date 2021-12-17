use std::ops::RangeInclusive;

use anyhow::Result;
use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", &self.x, &self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Probe {
    position: Point,
    velocity: Point,
}

impl Probe {
    fn new(velocity: Point) -> Self {
        Self {
            position: Point { x: 0, y: 0 },
            velocity,
        }
    }

    fn step(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        if self.velocity.x > 0 {
            self.velocity.x -= 1;
        } else if self.velocity.x < 0 {
            self.velocity.x += 1;
        }
        self.velocity.y -= 1;
    }
}

#[derive(Debug)]
struct TargetArea {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
}

impl TargetArea {
    fn from_input(s: &str) -> Result<Self> {
        let regex = Regex::new(r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)")?;
        let captures = regex.captures(s).ok_or(anyhow::anyhow!("bad input"))?;
        Ok(TargetArea {
            x: captures[1].parse()?..=captures[2].parse()?,
            y: captures[3].parse()?..=captures[4].parse()?,
        })
    }

    fn contains(&self, p: &Point) -> bool {
        self.x.contains(&p.x) && self.y.contains(&p.y)
    }
}

fn simulate(area: &TargetArea, velocity: Point) -> Option<isize> {
    let mut probe = Probe::new(velocity);
    let mut best_y = 0;
    while probe.position.x < *area.x.end() && probe.position.y >= *area.y.start() {
        probe.step();
        best_y = probe.position.y.max(best_y);
        if area.contains(&probe.position) {
            return Some(best_y);
        }
    }
    None
}

fn main() -> Result<()> {
    let input = include_str!("day17_input.txt").trim_end();
    let area = TargetArea::from_input(input)?;
    let mut possibilities = 0;
    let mut best_y = 0;
    for y in *area.y.start()..=area.y.start().abs() {
        for x in 0..=*area.x.end() {
            let velocity = Point { x, y };
            if let Some(max) = simulate(&area, velocity) {
                possibilities += 1;
                best_y = best_y.max(max);
            }
        }
    }

    println!("{:?}", best_y);
    println!("{:?}", possibilities);
    Ok(())
}
