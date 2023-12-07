use anyhow::anyhow;
use std::{cmp::Ordering, collections::HashMap, marker::PhantomData, str::FromStr};

mod permutation;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) enum Outcome {
    HighCard,
    Pair,
    TwoPair,
    Set,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl PartialOrd for Outcome {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Outcome {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl Outcome {
    fn value(&self) -> u8 {
        match self {
            Outcome::HighCard => 1,
            Outcome::Pair => 2,
            Outcome::TwoPair => 3,
            Outcome::Set => 4,
            Outcome::FullHouse => 5,
            Outcome::FourOfAKind => 6,
            Outcome::FiveOfAKind => 7,
        }
    }
}

pub(super) trait Rules {
    fn outcome(cards: &[Card; 5]) -> Outcome;

    fn card_value(card: &Card) -> u8;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(super) enum Card {
    // 2-10
    N(u8),
    Jack,
    Queen,
    King,
    As,
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '2' => Card::N(2),
            '3' => Card::N(3),
            '4' => Card::N(4),
            '5' => Card::N(5),
            '6' => Card::N(6),
            '7' => Card::N(7),
            '8' => Card::N(8),
            '9' => Card::N(9),
            'T' => Card::N(10),
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::As,
            _ => Err(anyhow!("invalid card"))?,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct Hand<R: Rules> {
    cards: [Card; 5],
    bid: u64,
    _phantom: PhantomData<R>,
}

impl<R: Rules + Eq> PartialOrd for Hand<R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Part1;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Part2;

impl Rules for Part1 {
    fn card_value(card: &Card) -> u8 {
        match card {
            Card::N(n) => *n,
            Card::Jack => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::As => 14,
        }
    }

    fn outcome(cards: &[Card; 5]) -> Outcome {
        let mut combos = HashMap::new();
        for card in cards {
            *combos.entry(card).or_insert(0usize) += 1;
        }

        let pairs = combos.values().filter(|&c| *c == 2).count();
        let two_pair = pairs == 2;
        let set = combos.values().filter(|&c| *c == 3).count();

        if pairs == 1 && set == 1 {
            return Outcome::FullHouse;
        }

        if two_pair {
            return Outcome::TwoPair;
        }

        if set == 1 {
            return Outcome::Set;
        }

        if pairs == 1 {
            return Outcome::Pair;
        }

        let four_of_a_kind = combos.values().filter(|&c| *c == 4).count();
        if four_of_a_kind == 1 {
            return Outcome::FourOfAKind;
        }

        let five_of_a_kind = combos.values().filter(|&c| *c == 5).count();
        if five_of_a_kind == 1 {
            return Outcome::FiveOfAKind;
        }

        return Outcome::HighCard;
    }
}

impl Rules for Part2 {
    fn card_value(card: &Card) -> u8 {
        match card {
            Card::N(n) => *n,
            Card::Jack => 1,
            Card::Queen => 12,
            Card::King => 13,
            Card::As => 14,
        }
    }

    fn outcome(cards: &[Card; 5]) -> Outcome {
        let jokers = cards
            .iter()
            .enumerate()
            .filter_map(|(idx, c)| (*c == Card::Jack).then_some(idx))
            .collect::<Vec<_>>();

        const POSSIBLE_CARDS: &'static [Card] = &[
            Card::N(2),
            Card::N(3),
            Card::N(4),
            Card::N(5),
            Card::N(6),
            Card::N(7),
            Card::N(8),
            Card::N(9),
            Card::N(10),
            Card::Jack,
            Card::Queen,
            Card::King,
            Card::As,
        ];

        let combinations =
            permutation::PermutationsWithReplacement::new(POSSIBLE_CARDS.iter(), jokers.len());

        let possible_cards = combinations.map(|combination| {
            let mut cards = cards.clone();

            for (joker_idx, card) in jokers.iter().zip(combination) {
                cards[*joker_idx] = *card;
            }

            cards
        });

        let outcome = possible_cards.map(|cards| Part1::outcome(&cards)).max();
        outcome.unwrap()
    }
}

impl<R: Rules + Eq> Ord for Hand<R> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.outcome(), other.outcome()) {
            (a, b) if a == b => {
                for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                    if R::card_value(c1) > R::card_value(c2) {
                        return Ordering::Greater;
                    }
                    if R::card_value(c2) > R::card_value(c1) {
                        return Ordering::Less;
                    }
                }

                Ordering::Equal
            }
            (a, b) => a.value().cmp(&b.value()),
        }
    }
}

impl<R: Rules> Hand<R> {
    fn outcome(&self) -> Outcome {
        R::outcome(&self.cards)
    }

    fn score(&self, rank: usize) -> u64 {
        self.bid * rank as u64
    }
}

impl<R: Rules> FromStr for Hand<R> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(" ").ok_or(anyhow!("invalid hand"))?;

        let cards = cards
            .chars()
            .map(Card::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let bid = bid.parse()?;

        Ok(Self {
            cards: cards
                .try_into()
                .map_err(|_| anyhow!("invalid number of cards"))?,
            bid,
            _phantom: PhantomData,
        })
    }
}

fn solve<R: Rules + Eq>(items: Vec<String>) -> anyhow::Result<u64> {
    let mut hands = items
        .into_iter()
        .map(|s| s.parse::<Hand<R>>())
        .collect::<Result<Vec<_>, _>>()?;

    hands.sort();

    let answer = hands
        .into_iter()
        .enumerate()
        .map(|(idx, hand)| hand.score(idx + 1))
        .sum();

    Ok(answer)
}

pub(super) struct Day7;
impl super::day::Day for Day7 {
    type Item = String;
    type Answer = u64;

    const DAY: usize = 7;

    fn part_1(items: Vec<Self::Item>) -> anyhow::Result<Self::Answer> {
        solve::<Part1>(items)
    }

    fn part_2(items: Vec<Self::Item>) -> anyhow::Result<Self::Answer> {
        solve::<Part2>(items)
    }
}
