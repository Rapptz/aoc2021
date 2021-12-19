use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
    str::FromStr,
};

use anyhow::{bail, Error, Result};
use itertools::Itertools;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

impl FromStr for Vector3 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let el: Vec<i32> = s.split(',').map(|p| p.parse()).collect::<Result<_, _>>()?;
        if el.len() != 3 {
            bail!("not enough elements")
        } else {
            Ok(Self {
                x: el[0],
                y: el[1],
                z: el[2],
            })
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Vector3 {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    #[inline]
    fn orientation(&self, index: u8) -> Vector3 {
        match index {
            0 => Self::new(self.x, self.y, self.z),
            1 => Self::new(self.x, self.z, -self.y),
            2 => Self::new(self.z, self.y, -self.x),
            3 => Self::new(self.x, -self.y, -self.z),
            4 => Self::new(-self.y, self.z, -self.x),
            5 => Self::new(self.z, -self.x, -self.y),
            6 => Self::new(-self.x, self.y, -self.z),
            7 => Self::new(self.x, -self.z, self.y),
            8 => Self::new(-self.z, -self.y, -self.x),
            9 => Self::new(-self.y, -self.x, -self.z),
            10 => Self::new(-self.x, self.z, self.y),
            11 => Self::new(self.z, -self.y, self.x),
            12 => Self::new(-self.x, -self.z, -self.y),
            13 => Self::new(-self.z, self.y, self.x),
            14 => Self::new(self.y, -self.z, -self.x),
            15 => Self::new(-self.z, -self.x, self.y),
            16 => Self::new(-self.x, -self.y, self.z),
            17 => Self::new(-self.y, -self.z, self.x),
            18 => Self::new(self.y, self.z, self.x),
            19 => Self::new(self.z, self.x, self.y),
            20 => Self::new(-self.z, self.x, -self.y),
            21 => Self::new(self.y, -self.x, self.z),
            22 => Self::new(-self.y, self.x, self.z),
            23 => Self::new(self.y, self.x, -self.z),
            _ => unreachable!(),
        }
    }

    fn rotate_in_place(&mut self, index: u8) {
        let mut rotation = self.orientation(index);
        std::mem::swap(self, &mut rotation);
    }
}

#[derive(Debug)]
struct Scanner {
    beacons: Vec<Vector3>,
    position: Vector3,
    index: usize,
}

impl Scanner {
    fn from_input(index: usize, s: &str) -> Self {
        Self {
            beacons: s.lines().skip(1).filter_map(|f| f.parse().ok()).collect(),
            index,
            position: Vector3::default(),
        }
    }
}

fn search(scanners: &mut Vec<Scanner>, scanner: usize, searched: &mut Vec<bool>) {
    searched[scanner] = true;

    for index in 0..scanners.len() {
        if searched[index] {
            continue;
        }

        // safety: index != scanner so no aliasing ptr
        let current = unsafe {
            let ptr = scanners.as_mut_ptr();
            &mut *ptr.add(index)
        };
        let start = &scanners[scanner];

        let mut distances: HashMap<Vector3, (u8, u32)> = HashMap::new();

        for other_beacon in &current.beacons {
            for orientation in 0..24 {
                let oriented = other_beacon.orientation(orientation);
                for beacon in &start.beacons {
                    let distance = *beacon - oriented;
                    distances
                        .entry(distance)
                        .and_modify(|f| f.1 += 1)
                        .or_insert((orientation, 1));
                }
            }
        }

        for (position, (orientation, count)) in distances {
            if count < 12 {
                continue;
            }
            for beacon in &mut current.beacons {
                beacon.rotate_in_place(orientation);
            }

            // scanners is being accessed by index to prevent reborrowing
            current.position = scanners[scanner].position + position;
            if !searched[current.index] {
                search(scanners, current.index, searched);
            }
        }
    }
}

fn solve(input: &str) -> Result<(usize, usize)> {
    let mut scanners: Vec<_> = input
        .split("\n\n")
        .enumerate()
        .map(|(i, s)| Scanner::from_input(i, s))
        .collect();

    let mut searched = vec![false; scanners.len()];
    search(&mut scanners, 0, &mut searched);
    let beacons: HashSet<_> = scanners
        .iter()
        .flat_map(|s| s.beacons.iter().copied().map(|l| l + s.position))
        .collect();

    let max = scanners
        .iter()
        .map(|s| s.position)
        .tuple_combinations()
        .map(|(p1, p2)| p1.distance(&p2))
        .max()
        .expect("unreachable");

    Ok((beacons.len(), max as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn test() {
        assert_eq!(solve(INPUT).unwrap(), (79, 3621));
    }
}

fn main() -> Result<()> {
    let input = include_str!("day19_input.txt").trim_end();
    println!("{:?}", solve(input)?);
    Ok(())
}
