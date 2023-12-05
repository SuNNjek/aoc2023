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

struct PuzzleMap<'a> {
    name: &'a str,
    entries: Vec<PuzzleMapEntry>,
}

impl<'a> PuzzleMap<'a> {
    fn parse_map(lines: &mut Lines<'a>) -> Option<PuzzleMap<'a>> {
        let (name, _) = lines.next()?.split_once(' ')?;
        let entries: Vec<PuzzleMapEntry> = lines
            .take_while(|&line| !line.is_empty())
            .filter_map(PuzzleMapEntry::from_string)
            .collect();

        Some(PuzzleMap { name, entries })
    }

    fn get_dest(&self, src: u64) -> u64 {
        let matching_entry = self.entries
            .iter()
            .filter(|&entry| entry.src_range_start <= src && src <= (entry.src_range_start + entry.range_len))
            .next();

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

struct Almanac<'a> {
    seeds: Vec<u64>,
    seed_to_soil: PuzzleMap<'a>,
    soil_to_fertilizer: PuzzleMap<'a>,
    fertilizer_to_water: PuzzleMap<'a>,
    water_to_light: PuzzleMap<'a>,
    light_to_temperature: PuzzleMap<'a>,
    temperature_to_humidity: PuzzleMap<'a>,
    humidity_to_location: PuzzleMap<'a>,
}

impl <'a> Almanac<'a> {
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
            seed_to_soil: maps.remove(0),
            soil_to_fertilizer: maps.remove(0),
            fertilizer_to_water: maps.remove(0),
            water_to_light: maps.remove(0),
            light_to_temperature: maps.remove(0),
            temperature_to_humidity: maps.remove(0),
            humidity_to_location: maps.remove(0),
        })
    }

    fn get_location_of_seed(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.get_dest(seed);
        let fert = self.soil_to_fertilizer.get_dest(soil);
        let water = self.fertilizer_to_water.get_dest(fert);
        let light = self.water_to_light.get_dest(water);
        let temp = self.light_to_temperature.get_dest(light);
        let hum = self.temperature_to_humidity.get_dest(temp);
        self.humidity_to_location.get_dest(hum)
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
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
