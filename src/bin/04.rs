advent_of_code::solution!(4);

use std::collections::{HashSet, HashMap, VecDeque};

#[derive(Clone)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers_you_have: HashSet<u32>,
}

impl Card {
    fn get_winning_count(&self) -> u32 {
        self.numbers_you_have.iter()
            .filter(|num| self.winning_numbers.contains(num))
            .count() as u32
    }

    fn get_score(&self) -> u32 {
        let winning_count = self.get_winning_count();

        if winning_count > 0 {
            u32::pow(2, winning_count - 1)
        } else {
            0
        }
    }

    fn get_copies<'a>(&self, originals: &'a HashMap<u32, &'a Card>) -> Vec<&'a Card> {
        let winning_count = self.get_winning_count();

        (0..winning_count).into_iter()
            .filter_map(|i| {
                let copy_id = self.id + i + 1;
                originals.get(&copy_id).and_then(|&c| Some(c))
            })
            .collect()
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

fn add_up_copies(cards: &mut VecDeque<Card>, originals: &HashMap<u32, &Card>) -> u32 {
    let mut count: u32 = 0;
    while let Some(card) = cards.pop_front() {
        count += 1;
        let copies = card.get_copies(originals);

        for copy in copies {
            cards.push_back(copy.clone())
        }
    }

    return count;
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
    let cards: Vec<Card> = input.lines()
       .filter_map(parse_card)
       .collect();

    let originals: HashMap<u32, &Card> = cards.iter()
        .map(|c| (c.id, c))
        .collect();

    let mut queue: VecDeque<Card> = cards.clone().into_iter().collect();

    Some(
        add_up_copies(&mut queue, &originals)
    )
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
        assert_eq!(result, Some(30));
    }
}
