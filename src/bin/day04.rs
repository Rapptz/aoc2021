use std::{collections::HashSet, str::FromStr};

use anyhow::{Result, bail};

struct Cell {
    value: u32,
    hit: bool,
}

impl Cell {
    fn new(value: u32) -> Self {
        Self { value, hit: false }
    }
}

struct Board {
    data: Vec<Cell>,
}

impl Board {
    const WIDTH: usize = 5;

    fn is_winner(&self) -> bool {
        for chunk in self.data.chunks(Self::WIDTH) {
            if chunk.iter().all(|c| c.hit) {
                return true;
            }
        }

        for column in 0..Self::WIDTH {
            if self
                .data
                .iter()
                .skip(column)
                .step_by(Self::WIDTH)
                .all(|f| f.hit)
            {
                return true;
            }
        }

        return false;
    }

    fn tick(&mut self, value: u32) {
        if let Some(c) = self.data.iter_mut().find(|c| c.value == value) {
            c.hit = true;
        }
    }

    fn unmarked_sum(&self) -> u32 {
        self.data.iter().filter(|c| !c.hit).map(|c| c.value).sum()
    }
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d: Result<Vec<_>, _> = s
            .split_whitespace()
            .map(|s| s.parse().map(Cell::new))
            .collect();
        Ok(Self { data: d? })
    }
}

fn get_lot(l: &str) -> Vec<u32> {
    l.split(',').filter_map(|s| s.parse().ok()).collect()
}

fn part1(input: &str) -> Result<u32> {
    let (lot, board) = input.split_once('\n').unwrap();
    let lot = get_lot(lot);
    let mut boards: Vec<Board> = board
        .trim_start()
        .split("\n\n")
        .filter_map(|f| f.parse().ok())
        .collect();

    for number in lot {
        for board in &mut boards {
            board.tick(number);
            if board.is_winner() {
                return Ok(board.unmarked_sum() * number);
            }
        }
    }
    bail!("unreachable")
}

fn part2(input: &str) -> Result<u32> {
    let (lot, board) = input.split_once('\n').unwrap();
    let lot = get_lot(lot);
    let mut boards: Vec<Board> = board
        .trim_start()
        .split("\n\n")
        .filter_map(|f| f.parse().ok())
        .collect();

    let count = boards.len();
    let mut winners = HashSet::new();
    for number in lot {
        for (index, board) in boards.iter_mut().enumerate() {
            board.tick(number);
            if board.is_winner() {
                winners.insert(index);
                if winners.len() == count {
                    return Ok(board.unmarked_sum() * number);
                }
            }
        }
    }
    bail!("unreachable")
}

fn main() -> Result<()> {
    let input = include_str!("day04_input.txt");
    println!("{:?}", part1(input)?);
    println!("{:?}", part2(input)?);
    Ok(())
}
