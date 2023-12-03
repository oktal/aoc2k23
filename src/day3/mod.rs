use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum RawPiece {
    Digit(u32),
    Char(char),
}

impl RawPiece {
    fn is_symbol(&self) -> bool {
        match self {
            Self::Char(c) if *c != '.' => true,
            _ => false,
        }
    }

    fn is_gear(&self) -> bool {
        matches!(self, Self::Char('*'))
    }
}

impl From<char> for RawPiece {
    fn from(value: char) -> Self {
        match value.to_digit(10) {
            Some(d) => RawPiece::Digit(d),
            _ => RawPiece::Char(value),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Piece {
    Number(u32, usize),
    Char(char),
}

impl Piece {
    fn is_symbol(&self) -> bool {
        match self {
            Piece::Char(c) if *c != '.' => true,
            _ => false,
        }
    }
}

fn lex(s: &str) -> Option<anyhow::Result<(Piece, &str)>> {
    let mut chars = s.char_indices();

    match chars.next() {
        Some((start, c)) if c.is_digit(10) => {
            let len = chars.by_ref().take_while(|(_, c)| c.is_digit(10)).count();
            let end = start + len + 1;
            Some(match u32::from_str_radix(&s[start..end], 10) {
                Ok(number) => Ok((Piece::Number(number, end - start), &s[end..])),
                Err(e) => Err(e.into()),
            })
        }
        Some((idx, c)) => Some(Ok((Piece::Char(c), &s[idx + 1..]))),
        None => None,
    }
}

#[derive(Debug, Clone)]
pub struct Fragment {
    pieces: Vec<Piece>,
    raw: String,
}

impl FromStr for Fragment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = Vec::new();
        let mut rest = s;
        while let Some(tok) = lex(rest) {
            let (piece, remainder) = tok?;
            pieces.push(piece);
            rest = remainder;
        }

        Ok(Self {
            pieces,
            raw: s.to_string(),
        })
    }
}

fn get_adjacent_indexes(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    const ADJACENT_MATRIX: &'static [(i32, i32)] = &[
        (0, -1),  // LEFT
        (0, 1),   // RIGHT
        (-1, 0),  // UP
        (1, 0),   // DOWN
        (-1, -1), // LEFT UP
        (-1, 1),  // RIGHT UP
        (1, -1),  // LEFT DOWN
        (1, 1),   // RIGHT DOWN
    ];

    ADJACENT_MATRIX
        .iter()
        .flat_map(move |(offset_x, offset_y)| {
            match (
                x.checked_add_signed(*offset_x as isize),
                y.checked_add_signed(*offset_y as isize),
            ) {
                (Some(x), Some(y)) => Some((x, y)),
                _ => None,
            }
        })
}

#[derive(Debug)]
pub(super) struct Engine {
    pieces: Vec<Piece>,
    columns: usize,
    raw: Vec<RawPiece>,
}

impl Engine {
    fn craft(fragments: Vec<Fragment>) -> anyhow::Result<Self> {
        let columns = fragments.first().ok_or(anyhow!("broken engine"))?.raw.len();
        let pieces = fragments
            .clone()
            .into_iter()
            .flat_map(|i| i.pieces)
            .collect::<Vec<_>>();
        let raw = fragments
            .clone()
            .into_iter()
            .map(|i| i.raw)
            .collect::<String>();
        let raw = raw.chars().map(RawPiece::from).collect();
        Ok(Engine {
            pieces,
            columns,
            raw,
        })
    }

    fn parts(&self) -> Vec<u32> {
        let mut parts = Vec::new();
        let mut raw_idx = 0usize;

        for piece in self.pieces.iter() {
            if let Piece::Number(n, len) = piece {
                let len = *len;

                let (row, column) = self.map_index(raw_idx);

                let mut adjacent_pieces = (0..len).flat_map(|y| {
                    get_adjacent_indexes(row, column + y).filter_map(|(x, y)| self.get_raw(x, y))
                });

                let is_part = adjacent_pieces.any(|p| p.is_symbol());
                if is_part {
                    parts.push(*n);
                }

                raw_idx += len;
            } else {
                raw_idx += 1;
            }
        }

        parts
    }

    fn gears(&self) -> Vec<u32> {
        let mut parts = HashMap::new();
        let mut raw_idx = 0usize;

        for piece in self.pieces.iter() {
            if let Piece::Number(n, len) = piece {
                let len = *len;
                let (row, column) = self.map_index(raw_idx);

                for y in 0..len {
                    let adjacent_gears = get_adjacent_indexes(row, column + y)
                        .filter_map(|(x, y)| self.get_raw(x, y).map(|piece| (piece, (x, y))))
                        .filter(|(p, _)| p.is_gear())
                        .collect::<Vec<_>>();

                    if !adjacent_gears.is_empty() {
                        for (_, index) in adjacent_gears {
                            parts.entry(index).or_insert(Vec::new()).push(*n);
                        }

                        break;
                    }
                }

                raw_idx += len;
            } else {
                raw_idx += 1;
            }
        }

        parts
            .into_values()
            .filter(|g| g.len() == 2)
            .map(|g| g.into_iter().product())
            .collect()
    }

    fn get_raw(&self, x: usize, y: usize) -> Option<RawPiece> {
        self.raw.get(x * self.columns + y).copied()
    }

    fn map_index(&self, idx: usize) -> (usize, usize) {
        (idx / self.columns, idx % self.columns)
    }
}

pub(super) struct Day3;
impl super::day::Day for Day3 {
    type Item = Fragment;
    type Answer = u32;

    const DAY: usize = 3;

    fn part_1(items: Vec<Self::Item>) -> anyhow::Result<Self::Answer> {
        let engine = Engine::craft(items)?;
        let parts = engine.parts();
        Ok(parts.into_iter().sum())
    }

    fn part_2(items: Vec<Self::Item>) -> anyhow::Result<Self::Answer> {
        let engine = Engine::craft(items)?;
        let gears = engine.gears();
        Ok(gears.into_iter().sum())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_symbol() {
        assert_eq!(Piece::Number(123, 3).is_symbol(), false);
        assert_eq!(Piece::Char('.').is_symbol(), false);
        assert_eq!(Piece::Char('$').is_symbol(), true);
    }

    #[test]
    fn lex_piece() {
        let s = "467..114..";
        let (vis, next) = lex(s).expect("number").unwrap();
        assert_eq!(vis, Piece::Number(467, 3));
        assert_eq!(next, "..114..");
    }

    #[test]
    fn parse() -> anyhow::Result<()> {
        let s = "467..114..";
        let fragment: Fragment = s.parse()?;
        assert_eq!(
            fragment.pieces,
            vec![
                Piece::Number(467, 3),
                Piece::Char('.'),
                Piece::Char('.'),
                Piece::Number(114, 3),
                Piece::Char('.'),
                Piece::Char('.'),
            ]
        );

        Ok(())
    }

    #[test]
    fn parse_with_number_last() -> anyhow::Result<()> {
        let s = "467..114..982";
        let fragment: Fragment = s.parse()?;
        assert_eq!(
            fragment.pieces,
            vec![
                Piece::Number(467, 3),
                Piece::Char('.'),
                Piece::Char('.'),
                Piece::Number(114, 3),
                Piece::Char('.'),
                Piece::Char('.'),
                Piece::Number(982, 3),
            ]
        );

        Ok(())
    }

    #[test]
    fn parse_with_single_digit_numbers() -> anyhow::Result<()> {
        let s = "467..114..982.4";
        let fragment: Fragment = s.parse()?;
        assert_eq!(
            fragment.pieces,
            vec![
                Piece::Number(467, 3),
                Piece::Char('.'),
                Piece::Char('.'),
                Piece::Number(114, 3),
                Piece::Char('.'),
                Piece::Char('.'),
                Piece::Number(982, 3),
                Piece::Char('.'),
                Piece::Number(4, 1),
            ]
        );

        Ok(())
    }

    #[test]
    fn adjacent() {
        assert_eq!(
            get_adjacent_indexes(0, 1).collect::<Vec<(_, _)>>(),
            vec![(0, 0), (0, 2), (1, 1), (1, 0), (1, 2)]
        );
    }
}
