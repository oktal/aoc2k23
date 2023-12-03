use std::str::FromStr;

use anyhow::{anyhow, bail};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => bail!("unknown color {s}"),
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Withdraw {
    count: u64,
    color: Color,
}

impl FromStr for Withdraw {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(' ');
        let count = parts
            .next()
            .ok_or(anyhow!("missing count"))
            .and_then(|c| c.parse().map_err(Into::into))?;

        let color = parts
            .next()
            .ok_or(anyhow!("missing count"))
            .and_then(|c| c.parse().map_err(Into::into))?;

        Ok(Self { count, color })
    }
}

impl Withdraw {
    fn is_possible(&self, bag: &Bag) -> bool {
        bag.count_for(self.color) >= self.count
    }
}

#[derive(Debug, Clone)]
struct Round(Vec<Withdraw>);

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let withdraws = s
            .trim()
            .split(',')
            .map(|w| w.parse().map_err(Into::into))
            .collect::<Result<Vec<_>, anyhow::Error>>()?;
        Ok(Self(withdraws))
    }
}

impl Round {
    fn is_possible(&self, bag: &Bag) -> bool {
        self.0.iter().all(|w| w.is_possible(bag))
    }
}

#[derive(Debug, Clone)]
pub(super) struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let game_start = parts.next().ok_or(anyhow!("Missing game identifier"))?;
        if !game_start.starts_with("Game ") {
            bail!("Game should start with `Game1`");
        }

        let game_id = game_start.replace("Game ", "");
        let game_id = game_id.parse()?;

        let rounds = parts
            .next()
            .ok_or(anyhow!("missing game rounds"))?
            .split(';')
            .map(|w| w.parse().map_err(Into::into))
            .collect::<Result<Vec<_>, anyhow::Error>>()?;
        Ok(Self {
            id: game_id,
            rounds,
        })
    }
}

impl Game {
    fn is_possible(&self, bag: &Bag) -> bool {
        self.rounds.iter().all(|r| r.is_possible(bag))
    }

    fn bag(&self) -> Bag {
        let red = self
            .rounds
            .iter()
            .flat_map(|r| r.0.iter().filter(|w| w.color == Color::Red))
            .map(|w| w.count)
            .max()
            .unwrap_or(0);

        let green = self
            .rounds
            .iter()
            .flat_map(|r| r.0.iter().filter(|w| w.color == Color::Green))
            .map(|w| w.count)
            .max()
            .unwrap_or(0);

        let blue = self
            .rounds
            .iter()
            .flat_map(|r| r.0.iter().filter(|w| w.color == Color::Blue))
            .map(|w| w.count)
            .max()
            .unwrap_or(0);

        Bag { red, green, blue }
    }
}

struct Bag {
    red: u64,
    green: u64,
    blue: u64,
}

impl Bag {
    fn count_for(&self, color: Color) -> u64 {
        match color {
            Color::Red => self.red,
            Color::Green => self.green,
            Color::Blue => self.blue,
        }
    }

    fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

pub(super) struct Day2;
impl super::day::Day for Day2 {
    type Item = Game;
    type Answer = u64;

    const DAY: usize = 2;

    fn part_1(games: Vec<Self::Item>) -> anyhow::Result<Self::Answer> {
        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };
        let answer = games
            .into_iter()
            .filter_map(|g| g.is_possible(&bag).then_some(g.id as u64))
            .sum();
        Ok(answer)
    }

    fn part_2(games: Vec<Self::Item>) -> anyhow::Result<Self::Answer> {
        let answer = games.into_iter().map(|g| g.bag().power()).sum();
        Ok(answer)
    }
}
