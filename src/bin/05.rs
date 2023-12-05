use std::str::Lines;

use itertools::Itertools;

advent_of_code::solution!(5);

struct PuzzleMapEntry {
    dest_range_start: u64,
    src_range_start: u64,
    range_len: u64,
}

impl PuzzleMapEntry {
    fn from_string(input: &str) -> Option<PuzzleMapEntry> {
        let (dest_range_start, src_range_start, range_len) = input
            .splitn(3, ' ')
            .filter_map(|part| part.parse().ok())
            .collect_tuple()?;

        Some(PuzzleMapEntry {
            dest_range_start,
            src_range_start,
            range_len
        })
    }
}

struct PuzzleMap {
    entries: Vec<PuzzleMapEntry>,
}

impl PuzzleMap {
    fn parse_map(lines: &mut Lines) -> Option<PuzzleMap> {
        lines.next()?;
        let entries: Vec<PuzzleMapEntry> = lines
            .take_while(|&line| !line.is_empty())
            .filter_map(PuzzleMapEntry::from_string)
            .collect();

        Some(PuzzleMap { entries })
    }

    fn get_dest(&self, src: u64) -> u64 {
        let matching_entry = self.entries
            .iter()
            .find(|&entry| entry.src_range_start <= src && src <= (entry.src_range_start + entry.range_len));

        match matching_entry {
            Some(&PuzzleMapEntry { src_range_start, dest_range_start, .. }) => dest_range_start + (src - src_range_start),
            None => src,
        }
    }
}

fn parse_seeds(input: &str) -> Option<Vec<u64>> {
    let (_, entries) = input.split_once(':')?;

    Some(
        entries.split_whitespace()
            .filter_map(|entry| entry.parse().ok())
            .collect()
    )
}

struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<PuzzleMap>,
}

impl Almanac {
    fn from_string(input: &str) -> Option<Almanac> {
        let mut lines = input.lines();

        let seeds = parse_seeds(lines.next()?)?;
        lines.next();
    
        let mut maps: Vec<PuzzleMap> = Vec::new();
        while let Some(map) = PuzzleMap::parse_map(&mut lines) {
            maps.push(map);
        }

        Some(Almanac {
            seeds,
            maps,
        })
    }

    fn get_location_of_seed(&self, seed: u64) -> u64 {
        self.maps.iter()
            .fold(seed, |value, map| map.get_dest(value))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let almanac = Almanac::from_string(input)?;

    almanac.seeds
        .iter()
        .map(|&s| almanac.get_location_of_seed(s))
        .min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let almanac = Almanac::from_string(input)?;

    let res = almanac.seeds.iter()
        .tuples::<(_, _)>()
        .flat_map(|(&start, &len)| {
            start..(start + len)
        })
        .map(|s| almanac.get_location_of_seed(s))
        .min();

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
