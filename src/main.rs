use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod parser;
use parser::{process, Grid, Outcome, Robot};

#[derive(Parser, Debug)]
struct Args {
    input: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    let outcomes = process_instructions(args.input);
    for outcome in outcomes {
        println!("{}", outcome);
    }
}

fn process_instructions<P>(filename: P) -> Vec<Outcome>
where
    P: AsRef<Path>,
{
    match read_lines(filename) {
        Ok(mut lines) => {
            let grid_str = lines.next();
            let grid = Grid::parse(grid_str.unwrap().unwrap());
            lines
                .flatten()
                .map(|line| Robot::parse(line))
                .map(|robot| process(&grid, &robot))
                .collect()
        }
        Err(e) => panic!("could not read file - {}", e),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_input_1() {
        let expected = vec![
            Outcome {
                x: 4,
                y: 4,
                direction: parser::Direction::East,
                is_lost: false,
            },
            Outcome {
                x: 0,
                y: 4,
                direction: parser::Direction::West,
                is_lost: true,
            },
        ];
        assert_eq!(expected, process_instructions("./input.txt"))
    }

    #[test]
    fn test_input_2() {
        let expected = vec![
            Outcome {
                x: 2,
                y: 3,
                direction: parser::Direction::West,
                is_lost: false,
            },
            Outcome {
                x: 1,
                y: 0,
                direction: parser::Direction::South,
                is_lost: true,
            },
        ];
        assert_eq!(expected, process_instructions("./input_2.txt"))
    }
}
