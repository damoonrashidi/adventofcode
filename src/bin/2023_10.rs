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
    let mut inside_x = false;
    let mut inside_y = false;

    for x in 0..map[0].len() {
        let c = &Coord(x, point.1);
        if path.contains(c) {
            inside_x = !inside_x;
        }
    }

    for y in 0..map.len() {
        if path.contains(&Coord(point.0, y)) {
            inside_y = !inside_y;
        }
    }

    inside_x && inside_y
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
    use std::collections::HashSet;

    use crate::{is_inside, parse, puzzle_one, puzzle_two, Coord};

    #[test]
    fn test_puzzle_one() {
        let map = parse(
            r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        );
        let actual = puzzle_one(&map, 'F');
        assert_eq!(actual, 8);
    }

    #[test]
    fn test_puzzle_two() {
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

    #[test]
    fn inside() {
        let map = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', 'S', '-', '7', '.'],
            vec!['.', '|', '.', '|', '.'],
            vec!['.', 'L', '-', 'J', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];
        let path: HashSet<Coord> = vec![
            Coord(1, 1),
            Coord(1, 2),
            Coord(1, 3),
            Coord(2, 3),
            Coord(3, 3),
            Coord(3, 2),
            Coord(3, 1),
            Coord(2, 1),
        ]
        .into_iter()
        .collect();
        assert!(!is_inside(Coord(2, 0), &path, &map));
        assert!(is_inside(Coord(2, 2), &path, &map));
    }
}
