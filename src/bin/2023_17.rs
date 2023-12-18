use std::{cmp::Ordering, collections::BinaryHeap};

fn main() {
    let input = include_str!("../../inputs/2023/17.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let map = parse(input);
    let mut dir = Orientation::Right;
    let mut pos = (0, 0);

    let goal = Coord(map.len() - 1, map[0].len() - 1);
    let start = Coord(0, 0);

    let mut closed: BinaryHeap<Node> = BinaryHeap::new();
    let mut open: BinaryHeap<Node> = BinaryHeap::new();

    open.push(Node::new(
        Coord(map.len() - 1, map[0].len() - 1),
        start.distance(&goal),
    ));

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let node = Node::new(Coord(x, y), Coord(x, y).distance(&start));
            closed.push(node);
        }
    }

    0
}

fn puzzle_two(input: &str) -> usize {
    let _map = parse(input);
    0
}

fn get_possible_nodes(coord: &Coord, direction: &Orientation, map: &[Vec<usize>]) -> Vec<Coord> {
    match direction {
        Orientation::Up => [coord.left(map), coord.down(map), coord.right(map)],
        Orientation::Right => [coord.up(map), coord.down(map), coord.right(map)],
        Orientation::Down => [coord.left(map), coord.right(map), coord.down(map)],
        Orientation::Left => [coord.left(map), coord.up(map), coord.down(map)],
    }
    .into_iter()
    .flatten()
    .collect()
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq)]
struct Coord(usize, usize);

#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: Coord,
    cost: usize,
}

impl Node {
    fn new(pos: Coord, cost: usize) -> Self {
        Self { pos, cost }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl Coord {
    pub(crate) fn up(&self, _map: &[Vec<usize>]) -> Option<Coord> {
        if self.1 > 0 {
            return Some(Coord(self.0, self.1 - 1));
        }
        None
    }
    pub(crate) fn right(&self, map: &[Vec<usize>]) -> Option<Coord> {
        if self.0 < map[0].len() - 1 {
            return Some(Coord(self.0 + 1, self.1));
        }
        None
    }
    pub(crate) fn down(&self, map: &[Vec<usize>]) -> Option<Coord> {
        if self.1 < map.len() - 1 {
            return Some(Coord(self.0, self.1 + 1));
        }
        None
    }
    pub(crate) fn left(&self, _map: &[Vec<usize>]) -> Option<Coord> {
        if self.0 > 0 {
            return Some(Coord(self.0 - 1, self.1));
        }
        None
    }

    pub(crate) fn distance(&self, other: &Coord) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::{puzzle_one, puzzle_two};

    #[test]
    fn test_puzzle_one() {
        let actual = puzzle_one(
            r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        );
        assert_eq!(actual, 102);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
