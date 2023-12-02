advent_of_code::solution!(2);

/// Revelation of cubes (R, G, B)
struct Revelation(u32, u32, u32);

struct Game(u32, Vec<Revelation>);

fn parse_revelation(input: &str) -> Revelation {
    input
        .split(",")
        .map(|p| p.trim())
        .filter_map(|s| {
            let (num_str, color) = s.split_once(" ")?;
            let num: u32 = num_str.parse().ok()?;

            Some((num, color))
        })
        .fold(
            Revelation(0, 0, 0),
            |Revelation(mut red, mut green, mut blue), (number, color)| {
                match color {
                    "red" => red = red + number,
                    "green" => green = green + number,
                    "blue" => blue = blue + number,

                    _ => (),
                }

                return Revelation(red, green, blue);
            },
        )
}

fn parse_game_def(input: &str) -> Option<u32> {
    let (game, num) = input.split_once(" ")?;
    if game != "Game" {
        return None;
    }

    num.parse().ok()
}

fn parse_game(input: &str) -> Option<Game> {
    let (game_def, revelations) = input.split_once(":")?;

    let game_num = parse_game_def(game_def)?;
    let revs: Vec<Revelation> = revelations
        .split(";")
        .map(|r| parse_revelation(r))
        .collect();

    return Some(Game(game_num, revs));
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines().filter_map(parse_game).collect()
}

fn game_fits(Game(_, game_revs): &Game, &Revelation(r, g, b): &Revelation) -> bool {
    game_revs
        .iter()
        .all(|&Revelation(total_r, total_g, total_b)| r >= total_r && g >= total_g && b >= total_b)
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_rev = Revelation(12, 13, 14);

    let fitting_games: Vec<Game> = parse_games(input)
        .into_iter()
        .filter(|g| game_fits(g, &total_rev))
        .collect();

    Some(fitting_games.into_iter().map(|Game(num, _)| num).sum())
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
