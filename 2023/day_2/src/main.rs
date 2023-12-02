#[derive(Debug, PartialEq)]
struct GameRound {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    rounds: Vec<GameRound>,
}

fn parse_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|l| {
            if l.starts_with("Game ") {
                Some(&l[5..])
            } else {
                unreachable!()
            }
        })
        .map(|l| {
            let mut parts = l.split(':');
            Game {
                id: parts.next().unwrap().parse().unwrap(),
                rounds: parts
                    .next()
                    .unwrap()
                    .split(';')
                    .map(|r| {
                        let parts = r.split(',');
                        let mut round = GameRound {
                            red: 0,
                            green: 0,
                            blue: 0,
                        };

                        for part in parts {
                            if part.ends_with(" red") {
                                round.red =
                                    part[..(part.chars().count() - 4)].trim().parse().unwrap();
                            }
                            if part.ends_with(" green") {
                                round.green =
                                    part[..(part.chars().count() - 6)].trim().parse().unwrap();
                            }
                            if part.ends_with(" blue") {
                                round.blue =
                                    part[..(part.chars().count() - 5)].trim().parse().unwrap();
                            }
                        }
                        round
                    })
                    .collect(),
            }
        })
        .collect()
}

fn find_min_requirments(game: &Game) -> GameRound {
    let mut min_req = GameRound {
        red: 0,
        green: 0,
        blue: 0,
    };

    for round in &game.rounds {
        min_req.red = min_req.red.max(round.red);
        min_req.green = min_req.green.max(round.green);
        min_req.blue = min_req.blue.max(round.blue);
    }

    min_req
}

fn invalid_with_cubes(games: &[Game], red: u32, green: u32, blue: u32) -> Vec<u32> {
    games
        .iter()
        .map(|game| (game.id, find_min_requirments(game)))
        .filter_map(|(game_id, min_round)| {
            if min_round.red <= red && min_round.green <= green && min_round.blue <= blue {
                Some(game_id)
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let parsed_input = parse_games(&input);

    let meet_req_sum: u32 = invalid_with_cubes(&parsed_input, 12, 13, 14).iter().sum();

    println!("Result 1: {meet_req_sum}");

    let min_req: u32 = parsed_input
        .iter()
        .map(find_min_requirments)
        .map(|r| r.red * r.green * r.blue)
        .sum();

    println!("Result 2: {min_req}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_parse() {
        let example = std::fs::read_to_string("./example.txt").unwrap();

        let parsed_example = parse_games(&example);

        println!("{:#?}", parsed_example);

        let expected = vec![
            Game {
                id: 1,
                rounds: vec![
                    GameRound {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    GameRound {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    GameRound {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ],
            },
            Game {
                id: 2,
                rounds: vec![
                    GameRound {
                        red: 0,
                        green: 2,
                        blue: 1,
                    },
                    GameRound {
                        red: 1,
                        green: 3,
                        blue: 4,
                    },
                    GameRound {
                        red: 0,
                        green: 1,
                        blue: 1,
                    },
                ],
            },
            Game {
                id: 3,
                rounds: vec![
                    GameRound {
                        red: 20,
                        green: 8,
                        blue: 6,
                    },
                    GameRound {
                        red: 4,
                        green: 13,
                        blue: 5,
                    },
                    GameRound {
                        red: 1,
                        green: 5,
                        blue: 0,
                    },
                ],
            },
            Game {
                id: 4,
                rounds: vec![
                    GameRound {
                        red: 3,
                        green: 1,
                        blue: 6,
                    },
                    GameRound {
                        red: 6,
                        green: 3,
                        blue: 0,
                    },
                    GameRound {
                        red: 14,
                        green: 3,
                        blue: 15,
                    },
                ],
            },
            Game {
                id: 5,
                rounds: vec![
                    GameRound {
                        red: 6,
                        green: 3,
                        blue: 1,
                    },
                    GameRound {
                        red: 1,
                        green: 2,
                        blue: 2,
                    },
                ],
            },
        ];

        assert_eq!(parsed_example, expected);

        let meet_req_sum: u32 = invalid_with_cubes(&parsed_example, 12, 13, 14).iter().sum();

        assert_eq!(meet_req_sum, 8);

        let min_req: u32 = parsed_example
            .iter()
            .map(find_min_requirments)
            .map(|r| r.red * r.green * r.blue)
            .sum();

        assert_eq!(min_req, 2286);
    }
}
