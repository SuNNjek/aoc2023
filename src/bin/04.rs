advent_of_code::solution!(4);

use std::collections::HashSet;

struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers_you_have: HashSet<u32>,
}

impl Card {
    fn get_score(&self) -> u32 {
        let winning_count = self.numbers_you_have.iter()
            .filter(|num| self.winning_numbers.contains(num))
            .count() as u32;

        if winning_count > 0 {
            u32::pow(2, winning_count - 1)
        } else {
            0
        }
    }
}

fn parse_game_num(input: &str) -> Option<u32> {
    let (_, num_str) = input.split_once(' ')?;
    num_str.trim().parse().ok()
}

fn parse_number_list(input: &str) -> HashSet<u32> {
    input.split(' ')
        .filter_map(|num_str| num_str.parse::<u32>().ok())
        .collect()
}

fn parse_card(input: &str) -> Option<Card> {
    let (def, numbers) = input.split_once(':')?;
    let (winning, have) = numbers.split_once('|')?;

    let game_num = parse_game_num(def)?;
    let winning_nums = parse_number_list(winning);
    let have_nums = parse_number_list(have);

    Some(Card {
        id: game_num,
        winning_numbers: winning_nums,
        numbers_you_have: have_nums
    })
}

/// For debug purposes
fn debug_input(input: &str) {
    input.lines().for_each(|line| {
        let card = parse_card(line);

        match card {
            Some(c) => println!("{}: Score {}", line, c.get_score()),
            None => println!("Failed to parse card: {}", line),
        }
    });
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input.lines()
            .filter_map(parse_card)
            .map(|c| c.get_score())
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
