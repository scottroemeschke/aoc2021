use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::str::FromStr;

fn main() -> Result<()> {
    println!("Reading input...");
    let raw_input = common::read_input()?;
    println!("Parsing input...");
    let parsed_input = parse_input(&raw_input);

    println!("Part One:\n");
    let answer = part_one(&parsed_input);
    println!(
        "Q: What do you get if you multiply your final horizontal position by your final depth?"
    );
    println!("A: {:?}", answer);

    println!("\n\nPart Two:\n");
    let answer = part_two(&parsed_input);
    println!(
        "Q: What do you get if you multiply your final horizontal position by your final depth?"
    );
    println!("A: {:?}", answer);

    Ok(())
}

struct SubPosition {
    depth: u64,
    horizontal_position: u64,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(Self::Err::msg(format!(
                "Failed to parse {} into valid direction",
                s
            ))),
        }
    }
}

#[derive(Debug, PartialEq)]
struct DirectionCommand {
    direction: Direction,
    value: u64,
}

impl FromStr for DirectionCommand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((direction, value)) = s.split(" ").collect_tuple() {
            let parsed_direction = direction.parse::<Direction>()?;
            let parsed_value = value.parse::<u64>()?;
            Ok(DirectionCommand {
                direction: parsed_direction,
                value: parsed_value,
            })
        } else {
            Err(Self::Err::msg(format!(
                "Failed to parse into valid DirectionCommand: ${}",
                s
            )))
        }
    }
}

fn parse_input(raw_input: &str) -> Vec<DirectionCommand> {
    raw_input
        .lines()
        .map(|s| s.trim())
        .map(|s| {
            s.parse::<DirectionCommand>()
                .with_context(|| format!("Failed to parse raw input: {}", raw_input))
                .unwrap() //panic if it doesn't parse
        })
        .collect()
}

fn part_one(commands: &[DirectionCommand]) -> u64 {
    let mut sub_position = SubPosition {
        horizontal_position: 0,
        depth: 0,
    };

    for command in commands {
        match command.direction {
            Direction::Forward => {
                sub_position.horizontal_position += command.value;
            }
            Direction::Down => {
                sub_position.depth += command.value;
            }
            Direction::Up => {
                sub_position.depth -= command.value;
            }
        }
    }

    sub_position.horizontal_position * sub_position.depth
}

fn part_two(commands: &[DirectionCommand]) -> u64 {
    let mut sub_position = SubPosition {
        horizontal_position: 0,
        depth: 0,
    };

    let mut sub_aim = 0;

    for command in commands {
        match command.direction {
            Direction::Forward => {
                sub_position.horizontal_position += command.value;
                sub_position.depth += command.value * sub_aim;
            }
            Direction::Down => {
                sub_aim += command.value;
            }
            Direction::Up => {
                sub_aim -= command.value;
            }
        }
    }

    sub_position.horizontal_position * sub_position.depth
}

#[cfg(test)]
mod tests {

    use crate::{parse_input, part_one, part_two, Direction, DirectionCommand};

    #[test]
    fn test_parsing() {
        let input = r#"
            forward 4
            down 2
            up 35
        "#
        .trim();
        let parsed = parse_input(&input);
        assert_eq!(
            parsed,
            vec![
                DirectionCommand {
                    direction: Direction::Forward,
                    value: 4
                },
                DirectionCommand {
                    direction: Direction::Down,
                    value: 2
                },
                DirectionCommand {
                    direction: Direction::Up,
                    value: 35
                }
            ]
        )
    }

    #[test]
    fn test_part_one() {
        let commands = [
            DirectionCommand {
                direction: Direction::Forward,
                value: 432,
            },
            DirectionCommand {
                direction: Direction::Down,
                value: 210,
            },
            DirectionCommand {
                direction: Direction::Forward,
                value: 34,
            },
            DirectionCommand {
                direction: Direction::Up,
                value: 11,
            },
            DirectionCommand {
                direction: Direction::Down,
                value: 0,
            },
        ];
        let answer = part_one(&commands);
        assert_eq!(answer, 92734); //calc by hand
    }

    #[test]
    fn test_part_two() {
        let commands = [
            DirectionCommand {
                direction: Direction::Forward,
                value: 432,
            },
            DirectionCommand {
                direction: Direction::Down,
                value: 210,
            },
            DirectionCommand {
                direction: Direction::Forward,
                value: 34,
            },
            DirectionCommand {
                direction: Direction::Up,
                value: 11,
            },
            DirectionCommand {
                direction: Direction::Down,
                value: 0,
            },
        ];
        let answer = part_two(&commands);
        assert_eq!(answer, 3327240); //calc by hand
    }
}
