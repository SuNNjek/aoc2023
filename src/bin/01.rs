advent_of_code::solution!(1);

use regex::{Regex, Match};

fn parse_digit(digit: &str) -> Option<u32> {
    match digit {
        "one"   | "1" => Some(1),
        "two"   | "2" => Some(2),
        "three" | "3" => Some(3),
        "four"  | "4" => Some(4),
        "five"  | "5" => Some(5),
        "six"   | "6" => Some(6),
        "seven" | "7" => Some(7),
        "eight" | "8" => Some(8),
        "nine"  | "9" => Some(9),

        _ => None,
    }
}

fn get_first_and_last_digit(line: &str, regex: Regex) -> Option<(u32, u32)> {
    // Needs to match at every possible position in the string, otherwise something like "5threeeightwor" will return 5 and eight,
    // probably because the "two" uses the "t" of "eight"
    let matches: Vec<Match> = (0..line.len()).filter_map(|i| regex.find_at(line, i)).collect();

    // Get the first and last matches and parse them as a string
    let first_digit = matches
        .first()
        .and_then(|m| parse_digit(m.as_str()));

    let last_digit = matches
        .last()
        .and_then(|m| parse_digit(m.as_str()));

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => Some((first, last)),
        _ => None
    }
}

// Callback that adds the given digits to the accumulator
fn sum_up_digits(acc: u32, digits: Option<(u32, u32)>) -> u32 {
    acc + match digits {
        Some((first, last)) => first * 10 + last,
        None => 0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let pattern = Regex::new("\\d").unwrap();

    Some(
        input.lines()
            .map(|l| get_first_and_last_digit(l, pattern.clone()))
            .fold(0, sum_up_digits)
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let pattern = Regex::new("\\d|one|two|three|four|five|six|seven|eight|nine").unwrap();

    Some(
        input.lines()
            .map(|l| get_first_and_last_digit(l, pattern.clone()))
            .fold(0, sum_up_digits)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
