use anyhow::Result;

#[inline]
fn score_p1(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

#[inline]
fn score_p2(c: &char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(),
    }
}

#[inline]
fn braces_match(open: char, close: char) -> bool {
    match (open, close) {
        ('(', ')') => true,
        ('<', '>') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        _ => false,
    }
}

fn illegal_score(l: &str) -> usize {
    let mut stack = vec![];
    for ch in l.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => {
                if let Some(top) = stack.pop() {
                    if !braces_match(top, ch) {
                        return score_p1(ch)
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    0
}

#[inline]
fn incomplete_score(s: Vec<char>) -> usize {
    s.iter().rev().fold(0, |acc, s| 5 * acc + score_p2(s))
}

fn fix_score(l: &str) -> Option<Vec<char>> {
    let mut stack = vec![];
    for ch in l.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => {
                if let Some(top) = stack.pop() {
                    if !braces_match(top, ch) {
                        return None;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    Some(stack)
}

fn part1(input: &str) -> Result<usize> {
    Ok(input.lines().map(illegal_score).sum())
}

fn part2(input: &str) -> Result<usize> {
    let mut scores: Vec<_> = input
        .lines()
        .filter_map(fix_score)
        .map(incomplete_score)
        .collect();
    scores.sort();
    let total = scores.len();
    Ok(scores[total / 2])
}

fn main() -> Result<()> {
    let input = include_str!("day10_input.txt").trim_end();
    println!("{:?}", part1(input)?);
    println!("{:?}", part2(input)?);
    Ok(())
}
