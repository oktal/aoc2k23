use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Millimeters(u64);
impl From<u64> for Millimeters {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Copy, Clone)]
struct Race {
    duration_ms: u64,
    distance: Millimeters,
}

impl Race {
    fn beats(&self, button_hold_duration_ms: u64) -> bool {
        let remaining_time = self.duration_ms - button_hold_duration_ms;
        let distance = button_hold_duration_ms * remaining_time;
        distance > self.distance.0
    }
}

#[derive(Debug)]
struct Number(u64);

impl FromStr for Number {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s
            .trim()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(digits.into_iter().fold(0, |acc, x| {
            acc * 10u64.pow((x as f64).log10() as u32 + 1) + x
        })))
    }
}

pub(super) struct Day6;
impl super::day::Day for Day6 {
    type Item = String;
    type Answer = usize;

    const DAY: usize = 6;

    fn part_1(items: Vec<Self::Item>) -> anyhow::Result<Self::Answer> {
        let time = items.get(0).ok_or(anyhow!("missing Time"))?;
        let (_, times) = time.split_once(":").ok_or(anyhow!("missing Time"))?;
        let times = times
            .trim()
            .split(" ")
            .filter(|d| !d.is_empty())
            .map(|t| t.parse());

        let distance = items.get(1).ok_or(anyhow!("missing Distance"))?;
        let (_, distances) = distance
            .split_once(":")
            .ok_or(anyhow!("missing Distance"))?;
        let distances = distances
            .trim()
            .split(" ")
            .filter(|d| !d.is_empty())
            .map(|t| t.parse::<u64>());

        let races = times
            .zip(distances)
            .map(|(time, distance)| {
                let time = time?;
                let distance = distance?;

                Ok::<_, anyhow::Error>(Race {
                    duration_ms: time,
                    distance: Millimeters::from(distance),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let answer = races
            .into_iter()
            .map(|r| (1..r.duration_ms - 1).filter(|d| r.beats(*d)).count())
            .product();

        Ok(answer)
    }

    fn part_2(items: Vec<Self::Item>) -> anyhow::Result<Self::Answer> {
        let time = items.get(0).ok_or(anyhow!("missing Time"))?;
        let (_, time) = time.split_once(":").ok_or(anyhow!("missing Time"))?;

        let distance = items.get(1).ok_or(anyhow!("missing Distance"))?;
        let (_, distance) = distance
            .split_once(":")
            .ok_or(anyhow!("missing Distance"))?;

        let time = Number::from_str(time)?;
        let distance = Number::from_str(distance)?;

        let race = Race {
            duration_ms: time
                .0
                .try_into()
                .map_err(|_| anyhow!("duration too long"))?,
            distance: Millimeters(
                distance
                    .0
                    .try_into()
                    .map_err(|_| anyhow!("distance too long"))?,
            ),
        };

        const MIN_BUTTON_HOLD_TIME_MS: u64 = 14;
        let max_button_hold_time_ms = race.duration_ms - MIN_BUTTON_HOLD_TIME_MS;

        let answer = (MIN_BUTTON_HOLD_TIME_MS..=max_button_hold_time_ms)
            .filter(|d| race.beats(*d))
            .count();
        Ok(answer)
    }
}
