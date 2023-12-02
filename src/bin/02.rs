advent_of_code::solution!(2);

use std::cmp::max;

struct Cubes {
    red_count: u32,
    green_count: u32,
    blue_count: u32,
}

impl Cubes {
    fn empty() -> Cubes {
        Cubes {
            red_count: 0,
            green_count: 0,
            blue_count: 0,
        }
    }

    fn get_power(&self) -> u32 {
        let &Cubes {
            red_count,
            green_count,
            blue_count,
        } = self;

        red_count * green_count * blue_count
    }
}

struct Game {
    id: u32,
    revelations: Vec<Cubes>,
}

fn parse_revelation(input: &str) -> Cubes {
    input
        .split(',')
        .map(|p| p.trim())
        .filter_map(|s| {
            let (num_str, color) = s.split_once(' ')?;
            let num: u32 = num_str.parse().ok()?;

            Some((num, color))
        })
        .fold(
            Cubes::empty(),
            |Cubes {
                 mut red_count,
                 mut green_count,
                 mut blue_count,
             },
             (number, color)| {
                match color {
                    "red" => red_count += number,
                    "green" => green_count += number,
                    "blue" => blue_count += number,

                    _ => (),
                }

                Cubes {
                    red_count,
                    green_count,
                    blue_count,
                }
            },
        )
}

fn parse_game_id(input: &str) -> Option<u32> {
    let (game, num) = input.split_once(' ')?;
    if game != "Game" {
        return None;
    }

    num.parse().ok()
}

fn parse_game(input: &str) -> Option<Game> {
    let (game_def, revelations) = input.split_once(':')?;

    let game_num = parse_game_id(game_def)?;
    let revs: Vec<Cubes> = revelations.split(';').map(parse_revelation).collect();

    Some(Game {
        id: game_num,
        revelations: revs,
    })
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines().filter_map(parse_game).collect()
}

fn game_fits(
    Game {
        revelations: game_revs,
        ..
    }: &Game,
    &Cubes {
        red_count,
        green_count,
        blue_count,
    }: &Cubes,
) -> bool {
    game_revs.iter().all(
        |&Cubes {
             red_count: rev_red,
             green_count: rev_green,
             blue_count: rev_blue,
         }| { red_count >= rev_red && green_count >= rev_green && blue_count >= rev_blue },
    )
}

fn get_minimum_cubes(
    Game {
        revelations: game_revs,
        ..
    }: &Game,
) -> Cubes {
    game_revs.iter().fold(
        Cubes::empty(),
        |Cubes {
             red_count: max_r,
             green_count: max_g,
             blue_count: max_b,
         },
         &Cubes {
             red_count: curr_r,
             green_count: curr_g,
             blue_count: curr_b,
         }| {
            Cubes {
                red_count: max(max_r, curr_r),
                green_count: max(max_g, curr_g),
                blue_count: max(max_b, curr_b),
            }
        },
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_cubes = Cubes {
        red_count: 12,
        green_count: 13,
        blue_count: 14,
    };

    Some(
        parse_games(input)
            .into_iter()
            .filter(|g| game_fits(g, &total_cubes))
            .map(|Game { id, .. }| id)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_games(input)
            .iter()
            .map(get_minimum_cubes)
            .map(|c| c.get_power())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
