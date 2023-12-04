use std::{collections::HashSet, str::FromStr};

use anyhow::anyhow;

#[derive(Debug, Clone)]
pub(super) struct ScratchCard {
    winning: HashSet<u32>,
    numbers: HashSet<u32>,
    copies: u32,
}

impl FromStr for ScratchCard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");
        let start = parts.next().ok_or(anyhow!("missing start"))?;

        let _card_id = start
            .strip_prefix("Card")
            .ok_or(anyhow!("card should start with Card"))?
            .trim();

        let content = parts.next().ok_or(anyhow!("numbers numbers"))?;
        let mut content = content.split("|");

        let winning = content.next().ok_or(anyhow!("missing winning numbers"))?;
        let winning = winning
            .trim()
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse())
            .collect::<Result<HashSet<_>, _>>()?;

        let numbers = content.next().ok_or(anyhow!("missing numbers"))?;
        let numbers = numbers
            .trim()
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse())
            .collect::<Result<HashSet<_>, _>>()?;

        Ok(Self {
            winning,
            numbers,
            copies: 0,
        })
    }
}

impl ScratchCard {
    fn winning_numbers<'a>(&'a self) -> impl Iterator<Item = u32> + 'a {
        self.numbers.intersection(&self.winning).copied()
    }

    fn count(&self) -> u32 {
        1 + self.copies
    }
}

pub(super) struct Day4;
impl super::day::Day for Day4 {
    type Item = ScratchCard;
    type Answer = u32;

    const DAY: usize = 4;

    fn part_1(cards: Vec<Self::Item>) -> anyhow::Result<Self::Answer> {
        let answer = cards
            .into_iter()
            .filter_map(|c| {
                let winning_numbers = c.winning_numbers().count() as u32;
                (winning_numbers > 0).then(|| 2u32.pow(winning_numbers - 1))
            })
            .sum();
        Ok(answer)
    }

    fn part_2(mut cards: Vec<Self::Item>) -> anyhow::Result<Self::Answer> {
        let copies = cards
            .iter()
            .map(|c| c.winning_numbers().count())
            .collect::<Vec<_>>();

        for (idx, copies) in copies.into_iter().enumerate() {
            let won_cards = cards
                .get_mut(idx + 1..idx + 1 + copies)
                .ok_or(anyhow!("missing cards"))?;

            for won_card in won_cards {
                won_card.copies += 1;
            }

            let current = &cards[idx];
            for _ in 0..current.copies {
                let won_cards = cards
                    .get_mut(idx + 1..idx + 1 + copies)
                    .ok_or(anyhow!("missing cards"))?;

                for won_card in won_cards {
                    won_card.copies += 1;
                }
            }
        }

        Ok(cards.into_iter().map(|c| c.count()).sum())
    }
}
