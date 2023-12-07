mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
use day::Part;
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day::solve::<Day1>("src/day1/input.txt", Part::All)?;
    day::solve::<Day2>("src/day2/input.txt", Part::All)?;
    day::solve::<Day3>("src/day3/input.txt", Part::All)?;
    day::solve::<Day4>("src/day4/input.txt", Part::All)?;
    day::solve::<Day5>("src/day5/input.txt", Part::One)?;
    day::solve::<Day6>("src/day6/input.txt", Part::All)?;
    day::solve::<Day7>("src/day7/input.txt", Part::All)?;
    Ok(())
}
