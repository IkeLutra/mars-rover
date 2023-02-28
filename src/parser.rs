use std::fmt::Display;

use regex::Regex;
#[derive(Debug, PartialEq)]
pub struct Grid {
    max_x: i32,
    max_y: i32,
}

impl Grid {
    pub fn parse(input: String) -> Self {
        let parsed: Vec<i32> = input
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        if parsed.len() != 2 {
            panic!("incorrect number of elements")
        }
        Grid {
            max_x: parsed[0],
            max_y: parsed[1],
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Forward,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::North => "N",
                Direction::East => "E",
                Direction::South => "S",
                Direction::West => "W",
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Robot {
    initial_x: i32,
    initial_y: i32,
    initial_orientation: Direction,
    commands: Vec<Command>,
}

impl Robot {
    pub fn parse(input: String) -> Self {
        let re = Regex::new(r"\((\d+), (\d+), ([NESW])\)\s+([[LRF]]+)").unwrap();
        let captures = re.captures(&input).unwrap();
        let x: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let y: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
        let direction = match captures.get(3).unwrap().as_str() {
            "N" => Direction::North,
            "S" => Direction::South,
            "E" => Direction::East,
            "W" => Direction::West,
            _ => panic!("impossible letter"),
        };
        let commands: Vec<Command> = captures
            .get(4)
            .unwrap()
            .as_str()
            .chars()
            .map(|c| match c {
                'L' => Command::Left,
                'R' => Command::Right,
                'F' => Command::Forward,
                _ => unimplemented!(),
            })
            .collect();
        Robot {
            initial_x: x,
            initial_y: y,
            initial_orientation: direction,
            commands: commands,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Outcome {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub is_lost: bool,
}

impl Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.direction)?;
        if self.is_lost {
            write!(f, " LOST")?;
        }
        Ok(())
    }
}

pub fn process(grid: &Grid, robot: &Robot) -> Outcome {
    let mut current_x = robot.initial_x;
    let mut current_y = robot.initial_y;
    let mut current_direction = robot.initial_orientation;
    let mut is_lost = false;
    for command in robot.commands.iter() {
        let (x, y, direction) = match command {
            Command::Forward => match current_direction {
                Direction::North => (current_x, current_y + 1, current_direction),
                Direction::East => (current_x + 1, current_y, current_direction),
                Direction::South => (current_x, current_y - 1, current_direction),
                Direction::West => (current_x - 1, current_y, current_direction),
            },
            Command::Left => match current_direction {
                Direction::North => (current_x, current_y, Direction::West),
                Direction::East => (current_x, current_y, Direction::North),
                Direction::South => (current_x, current_y, Direction::East),
                Direction::West => (current_x, current_y, Direction::South),
            },
            Command::Right => match current_direction {
                Direction::North => (current_x, current_y, Direction::East),
                Direction::East => (current_x, current_y, Direction::South),
                Direction::South => (current_x, current_y, Direction::West),
                Direction::West => (current_x, current_y, Direction::North),
            },
        };
        if x < 0 || x > grid.max_x || y < 0 || y > grid.max_y {
            is_lost = true;
            break;
        }
        current_x = x;
        current_y = y;
        current_direction = direction;
    }
    Outcome {
        x: current_x,
        y: current_y,
        direction: current_direction,
        is_lost: is_lost,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_grid() {
        let expected = Grid { max_x: 4, max_y: 8 };
        let actual = Grid::parse("4 8".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_robot_1() {
        let expected = Robot {
            initial_x: 2,
            initial_y: 3,
            initial_orientation: Direction::East,
            commands: vec![
                Command::Left,
                Command::Forward,
                Command::Right,
                Command::Forward,
                Command::Forward,
            ],
        };
        let actual = Robot::parse("(2, 3, E) LFRFF".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_process_1() {
        let robot = Robot {
            initial_x: 2,
            initial_y: 3,
            initial_orientation: Direction::East,
            commands: vec![
                Command::Left,
                Command::Forward,
                Command::Right,
                Command::Forward,
                Command::Forward,
            ],
        };
        let grid = Grid { max_x: 4, max_y: 8 };
        let outcome = process(&grid, &robot);
        assert_eq!(
            outcome,
            Outcome {
                x: 4,
                y: 4,
                direction: Direction::East,
                is_lost: false
            }
        );
    }

    #[test]
    fn test_process_2() {
        let robot = Robot {
            initial_x: 0,
            initial_y: 2,
            initial_orientation: Direction::North,
            commands: vec![
                Command::Forward,
                Command::Forward,
                Command::Left,
                Command::Forward,
                Command::Right,
                Command::Forward,
                Command::Forward,
            ],
        };
        let grid = Grid { max_x: 4, max_y: 8 };
        let outcome = process(&grid, &robot);
        assert_eq!(
            outcome,
            Outcome {
                x: 0,
                y: 4,
                direction: Direction::West,
                is_lost: true
            }
        );
    }
    #[test]
    fn test_display_outcome() {
        let outcome1 = Outcome {
            x: 1,
            y: 2,
            direction: Direction::South,
            is_lost: false,
        };
        assert_eq!(outcome1.to_string(), "(1, 2, S)");
        let outcome2 = Outcome {
            x: 3,
            y: 4,
            direction: Direction::West,
            is_lost: true,
        };
        assert_eq!(outcome2.to_string(), "(3, 4, W) LOST");
    }
}
