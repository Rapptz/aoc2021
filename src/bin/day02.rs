use std::str::FromStr;
use anyhow::{bail, Result};

#[derive(Debug, Default)]
struct Submarine {
    position: i32,
    depth: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Instruction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, value) = match s.split_once(' ') {
            Some(tup) => tup,
            None => bail!("bad input"),
        };
        let value: i32 = value.parse()?;
        match name {
            "forward" => return Ok(Self::Forward(value)),
            "up" => return Ok(Self::Up(value)),
            "down" => return Ok(Self::Down(value)),
            _ => bail!("unknown input"),
        }
    }
}

impl Submarine {
    fn process(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Forward(v) => self.position += v,
            Instruction::Up(v) => self.depth -= v,
            Instruction::Down(v) => self.depth += v,
        }
    }
}

fn part1(input: &str) -> Result<i32> {
    let mut submarine = Submarine::default();
    input
        .lines()
        .filter_map(|f| f.parse().ok())
        .for_each(|instruction| submarine.process(instruction));
    Ok(submarine.position * submarine.depth)
}

#[derive(Debug, Default)]
struct AimedSubmarine {
    position: i32,
    depth: i32,
    aim: i32,
}

impl AimedSubmarine {
    fn process(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Forward(v) => {
                self.position += v;
                self.depth += self.aim * v;
            }
            Instruction::Up(v) => self.aim -= v,
            Instruction::Down(v) => self.aim += v,
        }
    }
}

fn part2(input: &str) -> Result<i32> {
    let mut submarine = AimedSubmarine::default();
    input
        .lines()
        .filter_map(|f| f.parse().ok())
        .for_each(|instruction| submarine.process(instruction));
    Ok(submarine.position * submarine.depth)
}

fn main() -> Result<()> {
    let input = include_str!("day02_input.txt");
    println!("{:?}", part1(input)?);
    println!("{:?}", part2(input)?);
    Ok(())
}
