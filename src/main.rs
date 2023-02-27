use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod parser;
use parser::{Grid, Robot};

#[derive(Parser, Debug)]
struct Args {
    input: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    if let Ok(mut lines) = read_lines(args.input) {
        let grid_str = lines.next();
        let grid = Grid::parse(grid_str.unwrap().unwrap());
        let robots: Vec<Robot> = lines.flatten().map(|line| Robot::parse(line)).collect();
        for robot in robots {
            let (x, y, direction, is_lost) = robot.process(&grid);
            let lost_str = if is_lost { "LOST" } else { "" };
            println!("({}, {}, {}) {}", x, y, direction, lost_str);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
