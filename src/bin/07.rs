advent_of_code::solution!(7);

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, PartialOrd, Hash)]
struct Card(char);

const CARD_ORDER: &str = "23456789TJQKA";

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp_order(other, CARD_ORDER)
    }
}

impl Card {
    fn cmp_order(&self, other: &Self, order: &str) -> std::cmp::Ordering {
        let self_idx = order.find(self.0)
            .map(|idx| idx as i32)
            .unwrap_or(-1);

        let other_idx = order.find(other.0)
            .map(|idx| idx as i32)
            .unwrap_or(-1);

        self_idx.cmp(&other_idx)
    }
}

struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn from_string(input: &str) -> Option<Hand> {
        let (cards, bid) = input.split_whitespace()
            .collect_tuple()
            .and_then(|(cards, bid_str)| {
                Some((cards.chars().map(Card).collect_vec(), bid_str.parse().ok()?))
            })?;

        Some(Hand {
            cards,
            bid,
        })
    }

    fn get_hand_type(&self) -> HandType {
        let group_counts = self.cards.iter().counts();
    
        let sorted_counts = group_counts.iter()
            .sorted_by(|(_, &a), (_, &b)| a.cmp(&b).reverse())
            .collect_vec();
    
        // Thanks random reddit person for this approach
        // I wish I could figure this sorta thing out by myself but I can't unfortunately
        let diff = (*sorted_counts[0].1 as i32) - sorted_counts.len() as i32;

        match diff {
            4 => HandType::FiveOfAKind,
            2 => HandType::FourOfAKind,
            1 => HandType::FullHouse,
            0 => HandType::ThreeOfAKind,
            -1 => HandType::TwoPair,
            -2 => HandType::OnePair,

            _ => HandType::HighCard,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let hands = input.lines()
        .filter_map(Hand::from_string)
        .collect_vec();

    hands.iter().for_each(|h| {
        h.get_hand_type();
    });

    Some(
        hands.iter()
            .sorted_by(|&a, &b| {
                a.get_hand_type().cmp(&b.get_hand_type())
                    .then_with(|| a.cards.cmp(&b.cards))
            })
            .enumerate()
            .map(|(i, hand)| hand.bid * (i as u32 + 1))
            .sum()
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
