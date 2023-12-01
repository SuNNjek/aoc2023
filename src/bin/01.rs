advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| {
        let firstDigit = line
            .chars()
            .find(|c| -> bool { c.is_digit(10) })
            .and_then(|c| -> Option<u32> { c.to_digit(10) });

        let lastDigit = line
            .chars()
            .rev()
            .find(|c| -> bool { c.is_digit(10) })
            .and_then(|c| -> Option<u32> { c.to_digit(10) });

        match (firstDigit, lastDigit) {
            (Some(first), Some(last)) => Some(first * 10 + last),
            _ => None
        }
    }).fold(0, |acc, res| {
        acc + match res {
            Some(value) => value,
            None => 0,
        }
    }))
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
