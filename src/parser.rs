#[derive(Debug, PartialEq)]
struct Grid {
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_grid() {
        let expected = Grid { max_x: 4, max_y: 8 };
        let actual = Grid::parse("4 8".to_string());
        assert_eq!(expected, actual);
    }
}
