advent_of_code::solution!(3);

use regex::Regex;

enum TokenValue {
    Symbol(char),
    Number(u32),
}

impl TokenValue {
    fn from_string(input: &str) -> TokenValue {
        input.parse()
            .map_or(TokenValue::Symbol(input.chars().next().unwrap()), TokenValue::Number)
    }
}

struct Token {
    row: i32,
    col_start: i32,
    col_end: i32,
    value: TokenValue,
}

fn get_tokens_from_line(input: &str, row: u32, token_regex: &Regex) -> Vec<Token> {
    token_regex.find_iter(input)
        .map(|m| {
            Token {
                row: row as i32,
                col_start: m.start() as i32,
                col_end: m.end() as i32 - 1,
                value: TokenValue::from_string(m.as_str())
            }
        })
        .collect()
}



fn get_tokens(input: &str) -> Vec<Token> {
    let token_regex = Regex::new(
        "(\\d+)|[^\\d.]"
    ).unwrap();

    input.lines()
        .enumerate()
        .flat_map(|(row, line)| get_tokens_from_line(line, row as u32, &token_regex))
        .collect()
}

fn are_tokens_adjacent(symbol: &Token, number: &Token) -> bool {
    let is_correct_row: bool = (symbol.row - 1) <= number.row && number.row <= (symbol.row + 1);
    let is_correct_col = symbol.col_start >= (number.col_start - 1) && symbol.col_end <= (number.col_end + 1);

    is_correct_row && is_correct_col
}

fn is_part_number(all_tokens: &[Token], number: &Token) -> bool {
    all_tokens.iter()
        .filter(|&t| match t.value {
            TokenValue::Symbol(_) => true,
            TokenValue::Number(_) => false,
        })
        .any(|symbol| are_tokens_adjacent(symbol, number))
}

fn get_gears_sum(tokens: &[Token]) -> u32 {
    // Get all '*' symbols
    let candidates: Vec<&Token> = tokens.iter()
        .filter(|&t| matches!(t.value, TokenValue::Symbol('*')))
        .collect();

    candidates.into_iter()
        .filter_map(|c| {
            let numbers: Vec<u32> = tokens.iter()
                .filter_map(|t| match t.value {
                    TokenValue::Number(num) if are_tokens_adjacent(c, t)
                        => Some(num),

                    _ => None,
                })
                .collect();

            if numbers.len() == 2 {
                Some(numbers.iter().product::<u32>())
            } else {
                None
            }
        }).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let tokens = get_tokens(input);

    let res: u32 = tokens.iter()
        .filter(|&t| match t {
            Token { value: TokenValue::Symbol(_), .. } => false,
            Token { value: TokenValue::Number(_), .. } => is_part_number(&tokens, t)
        })
        .map(|t| match t.value {
            TokenValue::Symbol(_) => 0,
            TokenValue::Number(num) => num,
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let tokens = get_tokens(input);
    let gears_sum = get_gears_sum(&tokens);

    Some(gears_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
