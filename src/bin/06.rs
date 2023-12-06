advent_of_code::solution!(6);

use std::iter::zip;
use itertools::Itertools;

struct Race {
    time: u64,
    record: u64,
}

impl Race {
    #[allow(dead_code)]
    fn get_distance(&self, hold_time: i64) -> i64 {
        -(hold_time * hold_time) + (self.time as i64) * hold_time
    }

    #[allow(dead_code)]
    fn get_optimal_hold_time(&self) -> u64 {
        self.time / 2
    }

    fn beat_record_lower_bound(&self) -> u64 {
        let ftime = self.time as f64;
        let frec = self.record as f64;

        // Good old p-q formula. Takes me back, man ^^
        let bound = (ftime / 2.0) - ((ftime * ftime) / 4.0 - frec).sqrt();

        // Sneaky edge case: We need to beat the time, so if the lower bound lands on a whole number
        // we need to add 1 to it so we actually beat it.
        if bound.fract() == 0.0 {
            (bound as u64) + 1
        } else {
            bound.ceil() as u64
        }
    }

    fn beat_record_upper_bound(&self) -> u64 {
        let ftime = self.time as f64;
        let frec = self.record as f64;

        let bound = (ftime / 2.0) + ((ftime * ftime) / 4.0 - frec).sqrt();

        // Sneaky edge case: We need to beat the time, so if the upper bound lands on a whole number
        // we need to subtract 1 from it so we actually beat it.
        if bound.fract() == 0.0 {
            (bound as u64) - 1
        } else {
            bound.floor() as u64
        }
    }
}

fn get_numbers(line: &str) -> Option<Vec<u64>> {
    let (_, nums) = line.split_once(':')?;
    
    Some(
        nums.split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect()
    )
}

fn get_number(line: &str) -> Option<u64> {
    let (_, nums) = line.split_once(':')?;

    nums.split_whitespace()
        .collect::<String>()
        .parse()
        .ok()
}

fn parse_input(input: &str) -> Option<Vec<Race>> {
    let (times, records) = input.lines()
        .filter_map(get_numbers)
        .collect_tuple()?;

    Some(
        zip(times, records)
            .map(|(time, record)| Race {
                time,
                record,
            })
            .collect()
    )
}

fn parse_single_race(input: &str) -> Option<Race> {
    let (time, record) = input.lines()
        .filter_map(get_number)
        .collect_tuple()?;

    Some(Race { time, record })
}

pub fn part_one(input: &str) -> Option<u64> {
    let races = parse_input(input)?;

    Some(
        races.iter()
            .map(|r| {
                let upper_bound = r.beat_record_upper_bound();
                let lower_bound = r.beat_record_lower_bound();

                upper_bound - lower_bound + 1
            })
            .product()
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let race = parse_single_race(input)?;

    let lower_bound = race.beat_record_lower_bound();
    let upper_bound = race.beat_record_upper_bound();

    Some(upper_bound - lower_bound + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
