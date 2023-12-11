use std::collections::HashSet;

fn main() {
    let universe = expand_universe(include_str!("../../inputs/2023/11.txt").trim());
    let input = parse(&universe);

    println!("puzzle one: {}", puzzle_one(&input));
    println!("puzzle two: {}", puzzle_two(&input));
}

fn puzzle_one(galaxies: &Vec<Coord>) -> usize {
    let mut unique_pairs = HashSet::new();

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            unique_pairs.insert((galaxies[i], galaxies[j]));
        }
    }

    unique_pairs.iter().map(|(a, b)| a.distance(*b)).sum()
}

fn puzzle_two(input: &Vec<Coord>) -> usize {
    input.len()
}

fn parse(map: &[Vec<char>]) -> Vec<Coord> {
    let mut coords = vec![];

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '#' {
                coords.push(Coord(x, y));
            }
        }
    }

    coords
}

fn expand_universe(input: &str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for y in (0..map.len()).rev() {
        if map[y].clone().into_iter().all(|c| c == '.') {
            map.insert(y + 1, map[y].clone());
        }
    }

    for x in (0..map[0].len()).rev() {
        if map.iter().map(|r| r[x]).all(|c| c == '.') {
            #[allow(clippy::needless_range_loop)]
            for xs in 0..map.len() {
                map[xs].insert(x + 1, '.');
            }
        }
    }

    map
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord(usize, usize);

#[allow(unused)]
impl Coord {
    fn distance(&self, other: Coord) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::{expand_universe, parse, puzzle_one};

    #[test]
    fn test_puzzle_one() {
        let universe = expand_universe(
            r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        let map = parse(&universe);
        let actual = puzzle_one(&map);
        assert_eq!(actual, 374);
    }
}
