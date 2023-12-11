use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/2023/10.txt");
    let map = parse(input);

    println!("puzzle one: {}", puzzle_one(&map, Tile::Vertical));
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

fn puzzle_one(map: &Map, start_type: Tile) -> usize {
    let mut map = map.clone();
    let mut node = get_starting_pos(&map);
    let start = node;
    let mut last = start;
    let mut steps = 0;
    map[start.1][start.0] = start_type;

    while node != start || steps == 0 {
        let travesable_nodes: Vec<Coord> = get_connecting_nodes(node, &map)
            .into_iter()
            .filter(|n| n != &last)
            .collect();

        last = node;
        node = if let Some(next) = travesable_nodes.first() {
            *next
        } else {
            start
        };
        steps += 1;
    }

    steps / 2
}

#[allow(unused)]
fn puzzle_two(map: &Map) -> usize {
    let mut map = map.clone();
    let mut node = get_starting_pos(&map);
    let mut visits: HashSet<Coord> = HashSet::new();
    let start = node;
    let mut steps = 0;
    map[start.1][start.0] = Tile::Vertical;

    visits.insert(start);

    while node != start || steps == 0 {
        let travesable_nodes: Vec<Coord> = get_connecting_nodes(node, &map)
            .into_iter()
            .filter(|n| !visits.contains(n))
            .collect();

        if let Some(next) = travesable_nodes.first() {
            node = *next;
        } else {
            node = start;
        }

        visits.insert(node);
        steps += 1;
    }

    steps / 2
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
    #[allow(clippy::match_on_vec_items)]
    match map[coord.1][coord.0] {
        Tile::Vertical => vec![coord.up(), coord.down()],
        Tile::Horizontal => vec![coord.left(), coord.right()],
        Tile::NE => vec![coord.up(), coord.right()],
        Tile::NW => vec![coord.up(), coord.left()],
        Tile::SW => vec![coord.down(), coord.left()],
        Tile::SE => vec![coord.down(), coord.right()],
        _ => vec![],
    }
}

fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
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
        let actual = puzzle_one(&map, crate::Tile::SE);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_puzzle_two() {
        let map = parse(
            r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        );
        let actual = puzzle_one(&map, crate::Tile::SE);
        assert_eq!(actual, 8);
    }
}
