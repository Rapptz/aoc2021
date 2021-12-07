use anyhow::{anyhow, Result};
use itertools::Itertools;

fn part1(input: &str) -> Result<i32> {
    let mut numbers: Vec<i32> = input.split(',').filter_map(|f| f.parse().ok()).collect();
    numbers.sort();
    let middle = numbers[numbers.len() / 2];
    Ok(numbers.iter().map(|f| (f - middle).abs()).sum())
}

#[inline(always)]
fn sum(x: i32) -> i32 {
    return (x * (x + 1)) / 2;
}

fn part2(input: &str) -> Result<i32> {
    let mut numbers: Vec<i32> = input.split(',').filter_map(|f| f.parse().ok()).collect();
    numbers.sort();
    let middle = numbers[numbers.len() / 2];
    let result: i32 = numbers.iter().map(|f| sum((f - middle).abs())).sum();

    let (min, max) = numbers
        .iter()
        .copied()
        .minmax()
        .into_option()
        .ok_or(anyhow!("no minmax"))?;

    let rest = (min..=max)
        .map(|m| numbers.iter().map(|x| sum((x - m).abs())).sum())
        .min()
        .ok_or(anyhow!("not found"))?;

    Ok(result.min(rest))
}

fn main() -> Result<()> {
    let input = include_str!("day07_input.txt").trim_end();
    println!("{:?}", part1(input)?);
    println!("{:?}", part2(input)?);
    Ok(())
}
