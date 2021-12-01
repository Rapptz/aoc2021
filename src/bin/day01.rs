use anyhow::Result;

fn solution(input: &str) -> Result<i64> {
    Ok(input
        .split_ascii_whitespace()
        .filter_map(|f| f.parse().ok())
        .collect::<Vec<i64>>()
        .windows(2)
        .map(|a| (a[1] > a[0]) as i64)
        .sum())
}

fn solution2(input: &str) -> Result<i64> {
    Ok(input
        .split_ascii_whitespace()
        .filter_map(|f| f.parse().ok())
        .collect::<Vec<i64>>()
        .windows(4)
        .map(|a| (a[3] > a[0]) as i64)
        .sum())
}

fn main() -> Result<()> {
    let input = include_str!("day01_input.txt");
    println!("{:?}", solution(input)?);
    println!("{:?}", solution2(input)?);
    Ok(())
}
