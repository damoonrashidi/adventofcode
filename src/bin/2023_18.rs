use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/2023/18.txt").trim();

    let short_instructions = parse(input);

    println!(
        "puzzle one: {}",
        puzzle_one(short_instructions, Coord(1, 1))
    );
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(instructions: Vec<Instruction>, seed: Coord) -> usize {
    let mut visits: HashSet<Coord> = HashSet::new();

    let mut x = 0isize;
    let mut y = 0isize;

    for instruction in instructions {
        visits.insert(Coord(x, y));
        let (n_x, n_y) = match instruction {
            Instruction::Up(d) => (x, y - d),
            Instruction::Down(d) => (x, y + d),
            Instruction::Right(d) => (x + d, y),
            Instruction::Left(d) => (x - d, y),
        };

        let x_diff = n_x.abs_diff(x);
        let y_diff = n_y.abs_diff(y);

        let x_direction = match x.cmp(&n_x) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        };

        let y_direction = match y.cmp(&n_y) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        };

        for _ in 0..x_diff {
            x += x_direction;
            visits.insert(Coord(x, y));
        }

        for _ in 0..y_diff {
            y += y_direction;
            visits.insert(Coord(x, y));
        }
    }

    floodfill(seed, &mut visits);

    visits.len()
}

fn puzzle_two(input: &str) -> usize {
    let instructions = parse2(input);

    let mut visits = vec![];
    let mut x = 0isize;
    let mut y = 0isize;
    let mut rope_dist = 0;

    for instruction in instructions {
        visits.push(Coord(x, y));
        let (nx, ny) = match instruction {
            Instruction::Up(d) => (x, y - d),
            Instruction::Down(d) => (x, y + d),
            Instruction::Right(d) => (x + d, y),
            Instruction::Left(d) => (x - d, y),
        };

        rope_dist += x.abs_diff(nx);
        rope_dist += y.abs_diff(ny);

        x = nx;
        y = ny;
    }

    let interior = visits
        .chunks(2)
        .fold(0, |sum, coords| {
            let a = coords[0];
            let b = coords[1];

            sum + a.0 * b.1 - a.1 * b.0
        })
        .unsigned_abs();

    interior + rope_dist / 2 + 1
}

fn floodfill(seed: Coord, list: &mut HashSet<Coord>) {
    let mut stack: Vec<Coord> = vec![seed];

    while let Some(coord) = stack.pop() {
        if list.contains(&coord) {
            continue;
        }

        list.insert(coord);

        for neighbor in [coord.up(), coord.down(), coord.right(), coord.left()]
            .into_iter()
            .filter(|node| !list.contains(node))
        {
            stack.push(neighbor);
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split_whitespace().collect();
            let distance = split[1].parse().unwrap();
            let i = split[0];
            match i {
                "U" => Instruction::Up(distance),
                "R" => Instruction::Right(distance),
                "D" => Instruction::Down(distance),
                "L" => Instruction::Left(distance),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn parse2(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split_whitespace().collect();
            let color_str = split[2].replace(['(', ')', '#'], "");
            let color = color_str.get(..=4).unwrap();
            let distance = isize::from_str_radix(color, 16).unwrap();
            match color_str.chars().last() {
                Some('0') => Instruction::Right(distance),
                Some('1') => Instruction::Down(distance),
                Some('2') => Instruction::Left(distance),
                Some('3') => Instruction::Up(distance),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord(isize, isize);

impl Coord {
    pub(crate) fn up(&self) -> Self {
        Self(self.0, self.1 - 1)
    }
    pub(crate) fn right(&self) -> Self {
        Self(self.0 + 1, self.1)
    }
    pub(crate) fn down(&self) -> Self {
        Self(self.0, self.1 + 1)
    }
    pub(crate) fn left(&self) -> Self {
        Self(self.0 - 1, self.1)
    }
}

#[derive(Debug)]
enum Instruction {
    Up(isize),
    Right(isize),
    Down(isize),
    Left(isize),
}

#[cfg(test)]
mod tests {
    use crate::{parse, puzzle_one, puzzle_two};

    #[test]
    fn test_puzzle_one() {
        let instructions = parse(
            r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        );
        let actual = puzzle_one(instructions, crate::Coord(-1, -1));
        assert_eq!(actual, 62);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = puzzle_two(
            r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        );
        assert_eq!(actual, 952_408_144_115);
    }
}
