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
    
        let sorted_counts = group_counts.into_values()
            .sorted()
            .collect_vec();
    
        // There's only one element, so all cards were the same
        if sorted_counts.len() == 1 {
            HandType::FiveOfAKind
        } else if sorted_counts.contains(&4) {
            HandType::FourOfAKind
        } else if sorted_counts.contains(&3) {
            if sorted_counts.contains(&2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if sorted_counts.contains(&2) {
            if sorted_counts.iter().filter(|&count| count == &2).count() == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        } else {
            HandType::HighCard
        }
    }    
}

pub fn part_one(input: &str) -> Option<u32> {
    let hands = input.lines()
        .filter_map(Hand::from_string)
        .collect_vec();

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
