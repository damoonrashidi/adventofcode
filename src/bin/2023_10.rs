use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/2023/10.txt");
    let map = parse(input);

    println!("puzzle one: {}", puzzle_one(&map, '|'));
    println!("puzzle two: {}", puzzle_two(&map, '|'));
}

type Map = Vec<Vec<char>>;

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

fn puzzle_one(map: &Map, start_type: char) -> usize {
    let mut map = map.clone();
    let mut node = get_starting_pos(&map);
    let start = node;
    let mut last = start;
    let mut steps = 0;
    map[start.1][start.0] = start_type;

    loop {
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
        if node == start {
            break;
        }
    }

    (steps + 1) / 2
}

fn puzzle_two(map: &Map, start_type: char) -> usize {
    let mut map = map.clone();
    let mut node = get_starting_pos(&map);
    let start = node;
    let mut visits: HashSet<Coord> = HashSet::new();
    map[start.1][start.0] = start_type;
    visits.insert(start);

    loop {
        let travesable_nodes: Vec<Coord> = get_connecting_nodes(node, &map)
            .into_iter()
            .filter(|n| !visits.contains(n))
            .collect();

        visits.insert(node);
        if travesable_nodes.is_empty() {
            break;
        }

        node = if let Some(next) = travesable_nodes.first() {
            *next
        } else {
            start
        };
    }

    let mut insides = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !visits.contains(&Coord(x, y)) && is_inside(Coord(x, y), &visits, &map) {
                insides += 1;
            }
        }
    }

    insides
}

fn is_inside(point: Coord, path: &HashSet<Coord>, map: &Map) -> bool {
    let mut cross_left = 0;
    let mut cross_right = 0;

    for x in 0..point.0 {
        let c = &Coord(x, point.1);
        if path.contains(c) {
            cross_left += 1;
        }
    }
    if cross_left % 2 == 0 {
        return false;
    }

    for x in point.0..map[0].len() {
        let c = &Coord(x, point.1);
        if path.contains(c) {
            cross_right += 1;
        }
    }

    cross_right % 2 != 0
}

fn parse(input: &str) -> Map {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_starting_pos(map: &Map) -> Coord {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'S' {
                return Coord(x, y);
            }
        }
    }
    unreachable!()
}

fn get_connecting_nodes(coord: Coord, map: &Map) -> Vec<Coord> {
    #[allow(clippy::match_on_vec_items)]
    match map[coord.1][coord.0] {
        '|' => vec![coord.up(), coord.down()],
        '-' => vec![coord.left(), coord.right()],
        'L' => vec![coord.up(), coord.right()],
        'J' => vec![coord.up(), coord.left()],
        '7' => vec![coord.down(), coord.left()],
        'F' => vec![coord.down(), coord.right()],
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {

    use crate::{parse, puzzle_two};

    #[test]
    fn one() {
        let map = parse(
            r"....F----7...........
...S--J....|.........
...|.......|.........
...L-------J........",
        );

        let actual = puzzle_two(&map, 'F');
        assert_eq!(actual, 11);
    }

    #[test]
    fn two() {
        let map = parse(
            r"OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJIL-7OO
L--JOL7IIILJS7F-7L7O
OOOOF-JIIF7FJ|L7L7L7
OOOOL7IF7||L7|IL7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO",
        );

        let actual = puzzle_two(&map, 'F');
        assert_eq!(actual, 8);
    }
}
