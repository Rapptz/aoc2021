use std::collections::HashMap;

use anyhow::Result;

const INPUT: [u32; 2] = [6, 1];

struct Player {
    position: u32,
    score: u32,
}

impl Player {
    fn new(position: u32) -> Self {
        Self { position, score: 0 }
    }

    #[inline(always)]
    fn is_winner(&self) -> bool {
        self.score >= 1000
    }

    #[inline(always)]
    fn roll(&mut self, roll: u32) {
        self.position = ((self.position + roll - 1) % 10) + 1;
        self.score += self.position;
    }
}

fn part1() -> Result<usize> {
    let mut p1 = Player::new(INPUT[0]);
    let mut p2 = Player::new(INPUT[1]);

    let mut dice = 0;

    loop {
        let r1 = 3 * dice + 6;
        let r2 = 3 * dice + 15;
        dice += 3;

        p1.roll(r1);
        if p1.is_winner() {
            break;
        }

        dice += 3;
        p2.roll(r2);

        if p2.is_winner() {
            break;
        }
    }

    let loser = p1.score.min(p2.score);
    Ok(loser as usize * dice as usize)
}

type Wins = [usize; 2];
type PlayerState = (u32, u32, u32, u32, usize);
type Cache = HashMap<PlayerState, Wins>;

const ROLLS: [u32; 27] = [
    3, 4, 5, 4, 5, 6, 5, 6, 7, 4, 5, 6, 5, 6, 7, 6, 7, 8, 5, 6, 7, 6, 7, 8, 7, 8, 9,
];

fn quantum_dice(
    pos1: u32,
    pos2: u32,
    score1: u32,
    score2: u32,
    current_player: usize,
    cache: &mut Cache,
) -> Wins {
    if let Some(wins) = cache.get(&(pos1, pos2, score1, score2, current_player)).copied() {
        return wins;
    }

    let mut wins = Wins::default();

    for roll in ROLLS {
        let mut pos = [pos1, pos2];
        let mut score = [score1, score2];

        pos[current_player] = ((pos[current_player] + roll - 1) % 10) + 1;
        score[current_player] += pos[current_player];

        if score[current_player] >= 21 {
            wins[current_player] += 1;
        } else {
            let [p1, p2] = quantum_dice(
                pos[0],
                pos[1],
                score[0],
                score[1],
                1 - current_player,
                cache,
            );
            wins[0] += p1;
            wins[1] += p2;
        }
    }

    cache.insert((pos1, pos2, score1, score2, current_player), wins);
    wins
}

fn part2() -> Option<usize> {
    let mut cache = Cache::new();
    let winners = quantum_dice(INPUT[0], INPUT[1], 0, 0, 0, &mut cache);
    winners.iter().max().cloned()
}

fn main() -> Result<()> {
    println!("{:?}", part1()?);
    println!("{:?}", part2().unwrap());
    Ok(())
}
