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

    for y in 0..map.len() {
        floodfill(Coord(0, y), &mut visits, &map);
        floodfill(Coord(map[0].len() - 1, y), &mut visits, &map);
    }

    for x in 0..map[0].len() {
        floodfill(Coord(x, 0), &mut visits, &map);
        floodfill(Coord(x, map.len() - 1), &mut visits, &map);
    }

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if visits.contains(&Coord(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    map.len() * map[0].len() - visits.len()
}

fn floodfill(seed: Coord, list: &mut HashSet<Coord>, map: &Map) {
    let mut stack: Vec<Coord> = vec![seed];

    while let Some(coord) = stack.pop() {
        if list.contains(&coord) {
            continue;
        }

        list.insert(coord);

        let mut nodes = vec![];
        if seed.1 > 0 {
            nodes.push(coord.up());
        }
        if seed.1 < map.len() - 1 {
            nodes.push(coord.down());
        }
        if seed.0 > 0 {
            nodes.push(coord.left());
        }
        if seed.0 < map[0].len() - 1 {
            nodes.push(coord.right());
        }

        for neighbor in nodes.into_iter().filter(|node| !list.contains(node)) {
            if neighbor.0 > 0
                && neighbor.0 < map[0].len()
                && neighbor.1 > 0
                && neighbor.1 < map.len()
            {
                stack.push(neighbor);
            }
        }
    }
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
    fn two() {
        let map = parse(
            r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );

        let actual = puzzle_two(&map, 'F');
        assert_eq!(actual, 4);
    }
}
