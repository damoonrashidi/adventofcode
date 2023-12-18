use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/2023/18.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let instructions = parse(input);
    let mut seen: HashSet<Coord> = HashSet::new();

    let mut x = 0isize;
    let mut y = 0isize;

    let mut min_x = 0isize;
    let mut min_y = 0isize;
    let mut max_x = 0isize;
    let mut max_y = 0isize;

    for (instruction, _) in instructions {
        seen.insert(Coord(x, y));
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
            seen.insert(Coord(x, y));
        }

        for _ in 0..y_diff {
            y += y_direction;
            seen.insert(Coord(x, y));
        }

        min_x = x.min(min_y);
        max_x = x.max(max_x);
        min_y = y.min(min_y);
        max_y = y.max(max_y);
    }

    floodfill(Coord(2, 2), &mut seen);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if seen.contains(&Coord(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    seen.len()
}

fn floodfill(seed: Coord, list: &mut HashSet<Coord>) {
    let mut stack: Vec<Coord> = vec![seed];

    while let Some(coord) = stack.pop() {
        if list.contains(&coord) {
            continue;
        }

        println!("checking {coord:?}");

        list.insert(coord);

        for neighbor in [coord.up(), coord.down(), coord.right(), coord.left()]
            .into_iter()
            .filter(|node| !list.contains(node))
        {
            stack.push(neighbor);
        }
    }
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

fn parse(input: &str) -> Vec<(Instruction, String)> {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split_whitespace().collect();
            let distance = split[1].parse().unwrap();
            let i = split[0];
            let instruction = match i {
                "U" => Instruction::Up(distance),
                "R" => Instruction::Right(distance),
                "D" => Instruction::Down(distance),
                "L" => Instruction::Left(distance),
                _ => unreachable!(),
            };

            let color = split[2].replace(['(', ')'], "");

            (instruction, color)
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
    use crate::{puzzle_one, puzzle_two};

    #[test]
    fn test_puzzle_one() {
        let actual = puzzle_one(
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
        assert_eq!(actual, 62);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}

/*

#######
#.....#
###...#
..#...#
..#...#
###.###
#...#..
##..###
.#....#
.######
*/
