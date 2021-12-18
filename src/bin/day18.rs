use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SnailfishPair {
    number: usize,
    depth: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Snailfish {
    data: Vec<SnailfishPair>,
}

impl Snailfish {
    fn from_input(s: &str) -> Self {
        let mut depth = 0;
        let mut data = Vec::new();
        for ch in s.as_bytes() {
            match ch {
                b'[' => depth += 1,
                b']' => depth -= 1,
                b'0'..=b'9' => data.push(SnailfishPair {
                    number: (*ch - b'0') as usize,
                    depth,
                }),
                _ => {}
            }
        }
        Self { data }
    }

    fn explode(&mut self) -> Option<usize> {
        let index = self.data.iter().position(|p| p.depth > 4)?;

        if index > 0 {
            self.data[index - 1].number += self.data[index].number;
        }
        if index + 2 < self.data.len() {
            self.data[index + 2].number += self.data[index + 1].number
        }
        let pair = &mut self.data[index];
        pair.number = 0;
        pair.depth -= 1;
        Some(index + 1)
    }

    fn split(&mut self) -> Option<(usize, SnailfishPair)> {
        let index = self.data.iter().position(|p| p.number >= 10)?;
        let mut pair = &mut self.data[index];

        let half = pair.number as f64 / 2.0;
        let lower = half.floor() as usize;
        let upper = half.ceil() as usize;

        pair.number = lower;
        pair.depth += 1;
        Some((
            index + 1,
            SnailfishPair {
                number: upper,
                depth: pair.depth,
            },
        ))
    }

    fn reduce(&mut self) {
        loop {
            if let Some(index) = self.explode() {
                self.data.remove(index);
            } else if let Some((index, pair)) = self.split() {
                self.data.insert(index, pair);
            } else {
                break;
            }
        }
    }

    fn magnitude(mut self) -> usize {
        while self.data.len() > 1 {
            let max_depth = self.data.iter().map(|p| p.depth).max().unwrap_or_default();
            if max_depth == 0 {
                break;
            }

            match self.data.iter().position(|p| p.depth == max_depth) {
                Some(index) => {
                    self.data[index].number =
                        3 * self.data[index].number + 2 * self.data[index + 1].number;
                    self.data[index].depth -= 1;
                    self.data.remove(index + 1);
                }
                None => break,
            }
        }
        self.data[0].number
    }
}

impl std::ops::Add<Snailfish> for Snailfish {
    type Output = Snailfish;

    fn add(mut self, rhs: Snailfish) -> Self::Output {
        self.data.extend(rhs.data.into_iter());
        for pair in self.data.iter_mut() {
            pair.depth += 1;
        }
        self.reduce();
        self
    }
}

fn part1(input: &str) -> Option<usize> {
    let fish = input
        .lines()
        .map(Snailfish::from_input)
        .reduce(|a, b| a + b)?;
    Some(fish.magnitude())
}

fn part2(input: &str) -> Option<usize> {
    let fishes: Vec<_> = input.lines().map(Snailfish::from_input).collect();
    fishes
        .iter()
        .permutations(2)
        .map(|perm| {
            let x = perm[0].clone() + perm[1].clone();
            x.magnitude()
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test_part1() {
        assert_eq!(part1("[[1,2],[[3,4],5]]"), Some(143));
        assert_eq!(part1("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), Some(1384));
        assert_eq!(part1("[[[[1,1],[2,2]],[3,3]],[4,4]]"), Some(445));
        assert_eq!(part1("[[[[3,0],[5,3]],[4,4]],[5,5]]"), Some(791));
        assert_eq!(part1("[[[[5,0],[7,4]],[5,5]],[6,6]]"), Some(1137));
        assert_eq!(
            part1("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
            Some(3488)
        );
        assert_eq!(part1(INPUT), Some(4140));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 3993);
    }
}

fn main() -> Result<()> {
    let input = include_str!("day18_input.txt").trim_end();
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
    Ok(())
}
