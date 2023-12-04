use std::{
    fmt::{Debug, Display},
    io::{self, BufRead},
    path::Path,
    str::FromStr,
};

use anyhow::{anyhow, bail};

#[allow(dead_code)]
pub(super) enum Part {
    All,
    One,
    Two,
}

pub trait Day {
    type Item: FromStr + Clone;
    type Answer: Display;

    const DAY: usize;

    fn part_1(_items: Vec<Self::Item>) -> anyhow::Result<Self::Answer> {
        bail!("unsolved yet")
    }

    fn part_2(_items: Vec<Self::Item>) -> anyhow::Result<Self::Answer> {
        bail!("unsolved yet")
    }
}

fn read_lines(path: impl AsRef<Path>) -> anyhow::Result<Vec<String>> {
    let file = std::fs::File::open(path)?;

    let reader = io::BufReader::new(file);
    let mut ret = Vec::new();
    for line in reader.lines() {
        ret.push(line?)
    }

    Ok(ret)
}

pub(super) fn solve<D: Day>(file: impl AsRef<Path>, part: Part) -> anyhow::Result<()>
where
    <<D as Day>::Item as FromStr>::Err: Debug + Display,
{
    let day = D::DAY;

    let file = file.as_ref();
    let file_path = file
        .to_str()
        .ok_or(anyhow!("failed to determine fail path"))?;

    let items = read_lines(file)?
        .into_iter()
        .map(|l| l.parse())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| anyhow!("{e}"))?;

    match part {
        Part::One => {
            println!("Solving day {day} (part 1) [{file_path}]");
            match D::part_1(items) {
                Ok(answer) => println!("Answer {answer}"),
                Err(e) => println!("failed to solve: {e}"),
            };
        }
        Part::Two => {
            println!("Solving day {day} (part 2) [{file_path}]");
            match D::part_2(items) {
                Ok(answer) => println!("Answer {answer}"),
                Err(e) => println!("failed to solve: {e}"),
            };
        }
        Part::All => {
            println!("Solving day {day} [{file_path}]");

            match D::part_1(items.clone()) {
                Ok(answer) => println!("Answer for part 1: {answer}"),
                Err(e) => println!("failed to solve part 1: {e}"),
            };

            match D::part_2(items.clone()) {
                Ok(answer) => println!("Answer for part 2: {answer}"),
                Err(e) => println!("failed to solve part 2: {e}"),
            };
        }
    };
    Ok(())
}
