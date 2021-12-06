use anyhow::Result;

fn solve(input: &[u8], days: usize) -> usize {
    let mut counts = [0usize; 9];
    for d in input {
        counts[*d as usize] += 1;
    }

    for _ in 0..days {
        counts.rotate_left(1);
        counts[6] += counts[8];
    }
    counts.iter().sum()
}

fn part1(input: &str) -> Result<usize> {
    let fish: Vec<u8> = input
        .trim()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    Ok(solve(&fish, 80))
}

fn part2(input: &str) -> Result<usize> {
    let fish: Vec<u8> = input
        .trim()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    Ok(solve(&fish, 256))
}

fn main() -> Result<()> {
    let input = include_str!("day06_input.txt");
    println!("{:?}", part1(input)?);
    println!("{:?}", part2(input)?);
    Ok(())
}
