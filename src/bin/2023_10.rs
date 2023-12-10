use std::{collections::HashSet, fmt::Display};

fn main() {
    let input = include_str!("../../inputs/2023/10.txt");
    let map = parse(input);

    println!("puzzle one: {}", puzzle_one(&map));
    // println!("puzzle two: {}", puzzle_two(&map));
}

type Map = Vec<Vec<Tile>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ground,
    Vertical,
    Horizontal,
    Start,
    NE,
    NW,
    SW,
    SE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(usize, usize);

impl Coord {
    fn up(&self) -> Coord {
        Coord(self.0, self.1 - 1)
    }

    fn down(&self) -> Coord {
        Coord(self.0, self.1 + 1)
    }

    fn left(&self) -> Coord {
        Coord(self.0 - 1, self.1)
    }

    fn right(&self) -> Coord {
        Coord(self.0 + 1, self.1)
    }
}

fn puzzle_one(map: &Map) -> usize {
    let mut node = get_starting_pos(map);
    let mut visits: HashSet<Coord> = HashSet::new();
    let start = node;
    let mut steps = 0;
    let mut revisits: Vec<Coord> = vec![];

    visits.insert(start);

    println!("starting at {start:?}");

    while node != start || steps == 0 {
        let travesable_nodes: Vec<Coord> = get_connecting_nodes(node, map)
            .into_iter()
            .filter(|n| {
                if n == &start && steps <= 1 {
                    false
                } else {
                    !visits.contains(n) || n == &start
                }
            })
            .collect();

        #[allow(clippy::needless_range_loop)]
        for y in node.1.saturating_sub(20)..node.1 + 20 {
            for x in node.0.saturating_sub(40)..(node.0 + 40) {
                if Coord(x, y) == node {
                    print!("x");
                }
                if visits.contains(&Coord(x, y)) {
                    print!("*");
                } else if travesable_nodes.contains(&Coord(x, y)) {
                    let n = map[y][x];
                    print!("{n}");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        if let Some(next) = travesable_nodes.first() {
            node = *next;
        } else if let Some(next) = revisits.pop() {
            node = next;
        } else {
            println!("stopping at {node:?}");
            return steps / 2;
        }

        travesable_nodes
            .iter()
            .skip(1)
            .for_each(|future| revisits.push(*future));

        visits.insert(node);
        steps += 1;
    }
    println!("stopping at {node:?}");

    steps / 2
}

#[allow(unused)]
fn puzzle_two(_map: &Map) -> usize {
    0
}

fn get_starting_pos(map: &Map) -> Coord {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == Tile::Start {
                return Coord(x, y);
            }
        }
    }
    unreachable!()
}

fn get_connecting_nodes(coord: Coord, map: &Map) -> Vec<Coord> {
    get_surrounding_tiles(coord, map)
        .into_iter()
        .filter_map(|(node, tile)| match (node, tile) {
            (node, Tile::Horizontal) => {
                if (coord.0 > 0 && node == coord.left())
                    || (coord.0 < map.len() - 1 && node == coord.right())
                {
                    Some(node)
                } else {
                    None
                }
            }
            (node, Tile::Vertical) => {
                if (coord.1 > 0 && node == coord.up())
                    || (coord.1 < map.len() - 1 && node == coord.down())
                {
                    Some(node)
                } else {
                    None
                }
            }
            (node, Tile::SW) => {
                if (coord.1 > 0 && node == coord.up())
                    || (coord.0 < map.len() - 1 && node == coord.right())
                {
                    Some(node)
                } else {
                    None
                }
            }
            (node, Tile::SE) => {
                if (coord.1 > 0 && node == coord.up()) || (coord.0 > 0 && node == coord.left()) {
                    Some(node)
                } else {
                    None
                }
            }
            (node, Tile::NE) => {
                if (coord.1 < map.len() - 1 && node == coord.down())
                    || (coord.0 > 0 && node == coord.left())
                {
                    Some(node)
                } else {
                    None
                }
            }
            (node, Tile::NW) => {
                if (coord.1 < map.len() - 1 && node == coord.down())
                    || (coord.0 < map.len() - 1 && node == coord.right())
                {
                    Some(node)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect()
}

fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect()
}

fn get_surrounding_tiles(coord: Coord, map: &Map) -> Vec<(Coord, &Tile)> {
    let x_range = coord.0.saturating_sub(1)..=(map[0].len() - 1).min(coord.0 + 1);
    let y_range = coord.1.saturating_sub(1)..=(map.len() - 1).min(coord.1 + 1);

    x_range
        .flat_map(move |x| y_range.clone().map(move |y| (x, y)))
        .filter(|(x, y)| {
            (0..map.len()).contains(y)
                && (0..map[0].len()).contains(x)
                && (x, y) != (&coord.0, &coord.1)
        })
        .map(|(x, y)| (Coord(x, y), &map[y][x]))
        .collect()
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            '.' => Self::Ground,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Vertical => '|',
            Self::Horizontal => '-',
            Self::Ground => '.',
            Self::NE => 'L',
            Self::NW => 'J',
            Self::SW => '7',
            Self::SE => 'F',
            Self::Start => 'S',
        };
        write!(f, "{c}")
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse, puzzle_one};

    #[test]
    fn test_puzzle_one() {
        let map = parse(
            r".....
.S-7.
.|.|.
.L-J.
.....",
        );
        let actual = puzzle_one(&map);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_puzzle_one_two() {
        let map = parse(
            r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        );
        let actual = puzzle_one(&map);
        assert_eq!(actual, 8);
    }
}
