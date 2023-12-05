use std::{str::FromStr, time::Instant};

use anyhow::{anyhow, bail};

#[derive(Debug, Clone)]
struct Path {
    source: String,
    destination: String,
}

impl FromStr for Path {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("-");

        let source = parts.next().ok_or(anyhow!("missing source"))?.to_string();
        let to = parts.next().ok_or(anyhow!("missing destination"))?;
        if to != "to" {
            bail!("expected `to`, got` {to}`");
        }

        let destination = parts
            .next()
            .ok_or(anyhow!("missing destination"))?
            .to_string();

        Ok(Self {
            source,
            destination,
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct MapRange {
    destination_start: u64,
    source_start: u64,
    len: u64,
}

impl MapRange {
    fn map(&self, n: u64) -> Option<u64> {
        let (range_start, range_end) = self.range();
        (n >= range_start && n <= range_end).then(|| self.destination_start + n - range_start)
    }

    fn range(&self) -> (u64, u64) {
        (self.source_start, self.source_start + self.len)
    }
}

impl FromStr for MapRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let destination_start = parts
            .next()
            .ok_or(anyhow!("missing destination start"))
            .and_then(|s| s.parse().map_err(anyhow::Error::new))?;

        let source_start = parts
            .next()
            .ok_or(anyhow!("missing source start"))
            .and_then(|s| s.parse().map_err(anyhow::Error::new))?;

        let len = parts
            .next()
            .ok_or(anyhow!("missing length"))
            .and_then(|s| s.parse().map_err(anyhow::Error::new))?;

        Ok(Self {
            destination_start,
            source_start,
            len,
        })
    }
}

#[derive(Debug, Clone)]
struct Map {
    category: Path,
    ranges: Vec<MapRange>,
}

impl Map {
    fn map(&self, n: u64) -> Option<u64> {
        self.ranges.iter().find_map(|r| r.map(n))
    }
}

impl TryFrom<Vec<String>> for Map {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut items = value.into_iter();
        let map = items.next().ok_or(anyhow!("missing path"))?;
        let path = map
            .split(" ")
            .next()
            .ok_or(anyhow!("missing path"))
            .and_then(Path::from_str)?;

        let ranges = items.map(|i| i.parse()).collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            category: path,
            ranges,
        })
    }
}

#[derive(Clone)]
struct Almanac {
    maps: Vec<Map>,
}

impl Almanac {
    fn create(blocks: &[String]) -> anyhow::Result<Almanac> {
        let maps = blocks
            .split(|b| b.is_empty())
            .map(|b| b.into_iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let maps = maps
            .into_iter()
            .map(Map::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { maps })
    }

    fn map(&self, source: &str) -> Option<&Map> {
        self.maps.iter().find(|m| m.category.source == source)
    }

    fn resolve(&self, seed: u64, source: &str, destination: &str) -> u64 {
        let mut dest = seed;
        let mut next_map = source;

        while let Some(map) = self.map(next_map) {
            next_map = map.category.destination.as_str();

            if let Some(n) = map.map(dest) {
                dest = n;
            }

            // println!(
            //     "{} -> {} -> {dest} (-> {next_map})",
            //     map.category.source, map.category.destination
            // );

            if map.category.destination == destination {
                break;
            }
        }

        dest
    }
}

struct Seeds(Vec<u64>);

impl Seeds {
    fn ranges(&self) -> impl Iterator<Item = (u64, u64)> + '_ {
        self.0.chunks(2).map(|c| {
            let range_start = c.first().copied().unwrap();
            let range_len = c.last().copied().unwrap();
            (range_start, range_start + range_len)
        })
    }
}

impl FromStr for Seeds {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, seeds) = s.split_once(":").ok_or(anyhow!("missing seeds"))?;

        let seeds = seeds
            .trim()
            .split(" ")
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(seeds))
    }
}

struct Worker {
    id: usize,
    almanac: Almanac,
    range: (u64, u64),
}

impl Worker {
    fn run(self) -> u64 {
        println!(
            "Start working for range ({}, {})",
            self.range.0, self.range.1
        );

        let start = Instant::now();
        let mut last = start.elapsed();

        let seed_count = self.range.1 - self.range.0;

        (self.range.0..self.range.1)
            .enumerate()
            .map(|(idx, s)| {
                let elapsed = start.elapsed();
                if elapsed - last >= std::time::Duration::from_millis(500) {
                    let id = self.id;
                    let percent = idx as f64 * 100.0 / seed_count as f64;
                    println!("Worker #{id} [{elapsed:?}] resolved {percent:.2}%");
                    last = elapsed;
                }

                self.almanac.resolve(s, "seed", "location")
            })
            .min()
            .unwrap()
    }
}

pub(super) struct Day5;
impl super::day::Day for Day5 {
    type Item = String;
    type Answer = u64;

    const DAY: usize = 5;

    fn part_1(lines: Vec<Self::Item>) -> anyhow::Result<Self::Answer> {
        let mut lines = lines.into_iter();
        let seeds = Seeds::from_str(lines.next().ok_or(anyhow!("missing seeds"))?.as_str())?;

        lines.next().ok_or(anyhow!("missing blocks"))?;

        let almanac = Almanac::create(lines.as_slice())?;
        let lowest_location = seeds
            .0
            .iter()
            .map(|seed| almanac.resolve(*seed, "seed", "location"))
            .min()
            .ok_or(anyhow!("impossible to compute lowest location"))?;

        Ok(lowest_location)
    }

    fn part_2(lines: Vec<Self::Item>) -> anyhow::Result<Self::Answer> {
        let mut lines = lines.into_iter();
        let seeds = Seeds::from_str(lines.next().ok_or(anyhow!("missing seeds"))?.as_str())?;

        lines.next().ok_or(anyhow!("missing blocks"))?;

        let almanac = Almanac::create(lines.as_slice())?;

        let lowest_location = std::thread::scope(|s| {
            let workers = seeds.ranges().enumerate().map(|(idx, range)| Worker {
                id: idx,
                almanac: almanac.clone(),
                range,
            });

            let handles = workers.map(|w| s.spawn(|| w.run())).collect::<Vec<_>>();

            handles.into_iter().map(|h| h.join().unwrap()).min()
        })
        .ok_or(anyhow!("impossible to compute lowest location"))?;

        Ok(lowest_location)
    }
}
