import pathlib
import argparse
import requests

def get_session() -> str:
    with open('.session', 'r') as fp:
        return fp.read().strip()

def download(day: int) -> bytes:
    cookies = { 'session': get_session() }
    url = f'https://adventofcode.com/2021/day/{day}/input'
    resp = requests.get(url, cookies=cookies)
    resp.raise_for_status()
    return resp.content

source_template = """use anyhow::Result;

fn part1(input: &str) -> Result<()> {{
    Ok(())
}}

fn part2(input: &str) -> Result<()> {{
    Ok(())
}}

fn main() -> Result<()> {{
    let input = include_str!("{filename}").trim_end();
    println!("{{:?}}", part1(input)?);
    // println!("{{:?}}", part2(input)?);
    Ok(())
}}
"""

def make_day(day: int, input: bytes):
    path = pathlib.Path('src/bin/')
    path.mkdir(parents=True, exist_ok=True)
    filename = f'day{day:02}_input.txt'
    with open(path / filename, 'wb') as fp:
        fp.write(input)

    with open(f'src/bin/day{day:02}.rs', 'w') as fp:
        fp.write(source_template.format(filename=filename))

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('day', type=int, help='The day of the AoC puzzle')
    args = parser.parse_args()
    input = download(args.day)
    make_day(args.day, input)

if __name__ == '__main__':
    main()
