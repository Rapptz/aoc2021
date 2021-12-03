use anyhow::Result;

// warning: not good

fn part1(input: &str) -> Result<u64> {
    const BITS: usize = 12;
    let mut epsilon = String::with_capacity(BITS);
    let mut gamma = String::with_capacity(BITS);
    for index in 0..BITS {
        let mut common = [0u64; 2];
        for line in input.lines() {
            let byte = line.as_bytes()[index];
            common[(byte - b'0') as usize] += 1;
        }
        let (low, high) = if common[0] < common[1] {
            ('0', '1')
        } else {
            ('1', '0')
        };
        gamma.push(high);
        epsilon.push(low);
    }
    let gamma = u64::from_str_radix(gamma.as_str(), 2)?;
    // This can technically be a bitwise NOT of the gamma but it's simpler to do it this way
    let epsilon = u64::from_str_radix(epsilon.as_str(), 2)?;
    Ok(gamma * epsilon)
}

fn filter_vec(vec: &mut Vec<String>, low: u8, high: u8) {
    let mut index = 0;
    while vec.len() >= 2 {
        let mut common = [0u32; 2];
        for line in vec.iter() {
            let byte = line.as_bytes()[index];
            common[(byte - b'0') as usize] += 1;
        }
        let b = if common[0] <= common[1] { low } else { high };
        vec.retain(|f| f.as_bytes()[index] == b);
        index += 1;
    }
}

fn part2(input: &str) -> Result<u64> {
    let mut oxygen: Vec<String> = input.lines().map(String::from).collect();
    let mut co2 = oxygen.clone();
    filter_vec(&mut oxygen, b'1', b'0');
    filter_vec(&mut co2, b'0', b'1');

    let oxygen = u64::from_str_radix(oxygen[0].as_str(), 2)?;
    let co2 = u64::from_str_radix(co2[0].as_str(), 2)?;
    Ok(oxygen * co2)
}

fn main() -> Result<()> {
    let input = include_str!("day03_input.txt");
    println!("{:?}", part1(input)?);
    println!("{:?}", part2(input)?);
    Ok(())
}
