use anyhow::anyhow;

const DIGIT_RULES: &'static [(&'static str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

trait Digits {
    fn find(s: &str) -> Vec<u32>;
}

struct Part1;
impl Digits for Part1 {
    fn find(s: &str) -> Vec<u32> {
        s.chars().filter_map(|c| c.to_digit(10)).collect()
    }
}

struct Part2;
impl Digits for Part2 {
    fn find(s: &str) -> Vec<u32> {
        let mut digits = Vec::new();

        for rule in DIGIT_RULES {
            for idx in s.match_indices(rule.0) {
                digits.push((idx.0, rule.1));
            }
        }

        for (idx, c) in s.chars().enumerate() {
            if let Some(d) = c.to_digit(10) {
                digits.push((idx, d));
            }
        }

        digits.sort_by(|a, b| a.0.cmp(&b.0));
        digits.into_iter().map(|d| d.1).collect()
    }
}

fn combine_digits(first: u32, second: u32) -> u32 {
    first * 10 + second
}

fn solve<D: Digits>(lines: Vec<String>) -> anyhow::Result<u32> {
    let mut sum = 0;

    for line in lines {
        let digits = D::find(&line);

        let first_digit = digits
            .first()
            .copied()
            .ok_or(anyhow!("missing first digit"))?;
        let last_digit = digits
            .last()
            .copied()
            .ok_or(anyhow!("missing last digit"))?;

        let combined = combine_digits(first_digit, last_digit);

        sum += combined;
    }

    Ok(sum)
}

pub(super) struct Day1;
impl super::day::Day for Day1 {
    type Item = String;
    type Answer = u32;

    const DAY: usize = 1;

    fn part_1(items: Vec<Self::Item>) -> anyhow::Result<Self::Answer> {
        solve::<Part1>(items)
    }

    fn part_2(items: Vec<Self::Item>) -> anyhow::Result<Self::Answer> {
        solve::<Part2>(items)
    }
}
