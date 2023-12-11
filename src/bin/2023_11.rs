use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord(usize, usize);

#[allow(unused)]
impl Coord {
    fn distance(&self, other: Coord) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

fn main() {
    let input = include_str!("../../inputs/2023/11.txt").trim();

    println!("puzzle one: {}", puzzle(input, 2));
    println!("puzzle two: {}", puzzle(input, 1_000_000));
}

fn puzzle(input: &str, multiplier: usize) -> usize {
    let (galaxies, ys, xs) = parse(input);

    let mut unique_pairs = HashSet::new();

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            unique_pairs.insert((galaxies[i], galaxies[j]));
        }
    }

    unique_pairs
        .iter()
        .map(|(a, b)| {
            let x_r = a.0.min(b.0)..b.0.max(a.0);
            let y_r = a.1.min(b.1)..b.1.max(a.1);
            let crossed_x = xs.iter().filter(|x| x_r.contains(x)).count();
            let crossed_y = ys.iter().filter(|y| y_r.contains(y)).count();

            a.distance(*b) + (crossed_x + crossed_y) * (multiplier - 1)
        })
        .sum()
}

fn parse(input: &str) -> (Vec<Coord>, Vec<usize>, Vec<usize>) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut ys = vec![];
    (0..map.len()).for_each(|y| {
        if map[y].clone().iter().all(|c| c == &'.') {
            ys.push(y);
        }
    });

    let mut xs = vec![];
    for x in 0..map[0].len() {
        if map.iter().map(|r| r[x]).all(|c| c == '.') {
            xs.push(x);
        }
    }

    let mut galaxies: Vec<Coord> = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '#' {
                galaxies.push(Coord(x, y));
            }
        }
    }

    (galaxies, ys, xs)
}
