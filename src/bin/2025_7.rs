use std::fmt::Debug;

fn main() {
    let input = include_str!("../../inputs/2025/7.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

#[derive(PartialEq)]
enum Tile {
    Start,
    Space,
    Split,
    Beam,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "S"),
            Self::Space => write!(f, "."),
            Self::Split => write!(f, "^"),
            Self::Beam => write!(f, "|"),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            '.' => Self::Space,
            '^' => Self::Split,
            _ => unreachable!(),
        }
    }
}

fn puzzle_one(input: &str) -> usize {
    let mut map: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect();

    let mut splits = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            match map[y][x] {
                Tile::Start => {
                    map[y + 1][x] = Tile::Beam;
                }
                Tile::Space => {
                    if y > 0 && map[y - 1][x] == Tile::Beam {
                        map[y][x] = Tile::Beam;
                    }
                }
                Tile::Split => {
                    if map[y - 1][x] == Tile::Beam {
                        splits += 1;
                    }
                    map[y][x - 1] = Tile::Beam;
                    map[y][x + 1] = Tile::Beam;
                    if map[y + 1][x - 1] != Tile::Split {
                        map[y + 1][x - 1] = Tile::Beam;
                    }
                    if map[y + 1][x + 1] != Tile::Split {
                        map[y + 1][x + 1] = Tile::Beam;
                    }
                }
                Tile::Beam => {}
            }
        }
    }

    splits
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );
        assert_eq!(actual, 21);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
