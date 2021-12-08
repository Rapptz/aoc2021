use std::collections::BTreeSet;

use anyhow::Result;

fn part1(input: &str) -> Result<usize> {
    // 1, 4, 7, 8
    // 2, 4, 3, 7
    Ok(input
        .lines()
        .filter_map(|f| f.split_once('|'))
        .flat_map(|f| f.1.split_ascii_whitespace())
        .filter(|&s| match s.len() {
            2 | 4 | 3 | 7 => true,
            _ => false,
        })
        .count())
}

//   0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....
//
//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

fn decode_input<'a>(input: Vec<&'a str>) -> Vec<BTreeSet<u8>> {
    let mut result = vec![BTreeSet::new(); 10];
    let mut sixes = Vec::new();
    let mut fives = Vec::new();

    /*
        lengths:
        0 => 6,
        1 => 2,
        2 => 5,
        3 => 5,
        4 => 4,
        5 => 5,
        6 => 6,
        7 => 3,
        8 => 7
        9 => 6

        ordered by length the numbers are 1, 7, 4, 3, 2, 5, 6, 9, 0, 8

        1 => cf
        7 => acf (a can be found out from that one)
        3 => acdfg (d and g need to be found out)
        4 => bcdf (b and d need to be found out)
        common between 3 and 4 is d, uncommon can be solved
        that solves ABDG, 5 gives you F, 6 gives you E, 1 gives you C
        then ABCDEFG are all found
    */

    // 1, 4, 7, 8 are unique
    for code in input.iter() {
        let digits: BTreeSet<u8> = code.bytes().collect();
        match code.len() {
            2 => result[1] = digits,
            3 => result[7] = digits,
            4 => result[4] = digits,
            5 => fives.push(digits),
            6 => sixes.push(digits),
            7 => result[8] = digits,
            _ => unreachable!(),
        };
    }

    // differentiate between the different 6s
    // the different 6 length digits are 0, 6, and 9
    // between 1 and 6 the only difference is C
    // between 4 and 0 the only difference is D
    // otherwise, it must be 9
    for digit in sixes {
        if !digit.is_superset(&result[1]) {
            result[6] = digit;
        } else if !digit.is_superset(&result[4]) {
            result[0] = digit;
        } else {
            result[9] = digit;
        }
    }

    // differentiate between the different 5s
    // the different 5 lengths are 2, 3, and 5
    // 6 is only a superset of 5 so it's easy to find
    // 3 is only a subset of 9 so it's easy to find
    // the last one has to be 2
    for digit in fives {
        if result[6].is_superset(&digit) {
            result[5] = digit;
        } else if result[9].is_superset(&digit) {
            result[3] = digit;
        } else {
            result[2] = digit;
        }
    }

    result
}

fn part2(input: &str) -> Result<usize> {
    let answer = input
        .lines()
        .filter_map(|s| s.split_once(" | "))
        .map(|(input, output)| {
            let decoded = decode_input(input.split_ascii_whitespace().collect());
            output.split_ascii_whitespace().fold(0, |total, s| {
                let digits: BTreeSet<u8> = s.bytes().collect();
                let digit = decoded
                    .iter()
                    .position(|d| *d == digits)
                    .unwrap_or_default();
                total * 10 + digit
            })
        })
        .sum();
    Ok(answer)
}

fn main() -> Result<()> {
    let input = include_str!("day08_input.txt").trim_end();
    println!("{:?}", part1(input)?);
    println!("{:?}", part2(input)?);
    Ok(())
}
