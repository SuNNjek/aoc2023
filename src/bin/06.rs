advent_of_code::solution!(6);

use std::iter::zip;
use itertools::Itertools;

struct Race {
    time: u32,
    record: u32,
}

impl Race {
    #[allow(dead_code)]
    fn get_distance(&self, hold_time: i32) -> i32 {
        -(hold_time * hold_time) + (self.time as i32) * hold_time
    }

    #[allow(dead_code)]
    fn get_optimal_hold_time(&self) -> u32 {
        self.time / 2
    }

    fn beat_record_lower_bound(&self) -> u32 {
        let ftime = self.time as f32;
        let frec = self.record as f32;

        // Good old p-q formula. Takes me back, man ^^
        let bound = (ftime as f32 / 2.0) - ((ftime * ftime) / 4.0 - frec).sqrt();

        // Sneaky edge case: We need to beat the time, so if the lower bound lands on a whole number
        // we need to add 1 to it so we actually beat it.
        if bound.fract() == 0.0 {
            (bound as u32) + 1
        } else {
            bound.ceil() as u32
        }
    }

    fn beat_record_upper_bound(&self) -> u32 {
        let ftime = self.time as f32;
        let frec = self.record as f32;

        let bound = (ftime as f32 / 2.0) + ((ftime * ftime) / 4.0 - frec).sqrt();

        // Sneaky edge case: We need to beat the time, so if the upper bound lands on a whole number
        // we need to subtract 1 from it so we actually beat it.
        if bound.fract() == 0.0 {
            (bound as u32) - 1
        } else {
            bound.floor() as u32
        }
    }
}

fn get_numbers(line: &str) -> Option<Vec<u32>> {
    let (_, nums) = line.split_once(':')?;
    
    Some(
        nums.split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect()
    )
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

pub fn part_one(input: &str) -> Option<u32> {
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

pub fn part_two(#[allow(unused_variables)] input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
