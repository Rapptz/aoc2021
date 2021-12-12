use std::collections::{HashMap, HashSet};

use anyhow::Result;

#[derive(Debug)]
struct Graph {
    inner: HashMap<String, Vec<String>>,
}

#[inline]
fn is_small(a: &str) -> bool {
    a.chars().all(|x| x.is_ascii_lowercase())
}

#[derive(Debug)]
struct Traversal<'a> {
    node: &'a str,
    path: HashSet<&'a str>,
    second: bool,
}

impl<'a> Traversal<'a> {
    fn new(node: &'a str, second: bool) -> Self {
        let mut path = HashSet::with_capacity(1);
        path.insert(node);
        Self { path, node, second }
    }

    fn augment(&self, node: &'a str, second: bool) -> Self {
        let mut path = self.path.clone();
        path.insert(node);
        Self { node, path, second }
    }
}

impl Graph {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add_edge(&mut self, left: String, right: String) {
        self.inner.entry(left).or_default().push(right);
    }

    fn from_input(s: &str) -> Graph {
        let mut graph = Self::new();
        s.lines()
            .filter_map(|s| s.split_once('-'))
            .for_each(|(begin, end)| {
                if end != "start" {
                    graph.add_edge(begin.to_owned(), end.to_owned());
                }
                if begin != "start" {
                    graph.add_edge(end.to_owned(), begin.to_owned());
                }
            });

        graph
    }

    fn paths<'a>(&'a self, second: bool) -> usize {
        let mut paths = 0;
        let mut stack: Vec<Traversal<'a>> = vec![Traversal::new("start", second)];

        while let Some(traversal) = stack.pop() {
            if let Some(children) = self.inner.get(traversal.node) {
                for child in children {
                    if child == "end" {
                        // dbg!(&traversal);
                        paths += 1;
                    } else if is_small(child.as_str()) && traversal.path.contains(child.as_str()) {
                        if !traversal.second {
                            stack.push(traversal.augment(child, true));
                        }
                    } else {
                        stack.push(traversal.augment(child, traversal.second));
                    }
                }
            }
        }
        paths
    }
}

fn part1(input: &str) -> Result<usize> {
    let graphs = Graph::from_input(input);
    Ok(graphs.paths(true))
}

fn part2(input: &str) -> Result<usize> {
    let graphs = Graph::from_input(input);
    Ok(graphs.paths(false))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 36);
    }
}

fn main() -> Result<()> {
    let input = include_str!("day12_input.txt").trim_end();
    println!("{:?}", part1(input)?);
    println!("{:?}", part2(input)?);
    Ok(())
}
