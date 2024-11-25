use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/2016/1.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

enum Dir {
    R(isize),
    L(isize),
}

enum Face {
    N,
    E,
    S,
    W,
}

fn parse(input: &str) -> Vec<Dir> {
    input
        .split(", ")
        .map(|c| {
            let len = c[1..].parse::<isize>().unwrap();
            if c.starts_with('L') {
                Dir::L(len)
            } else {
                Dir::R(len)
            }
        })
        .collect()
}

fn puzzle_one(input: &str) -> isize {
    let instructions = parse(input);
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut d = Face::N;

    for i in instructions {
        let (dx, dy, nd) = match (d, i) {
            (Face::S, Dir::R(dist)) | (Face::N, Dir::L(dist)) => (-dist, 0, Face::W),
            (Face::N, Dir::R(dist)) | (Face::S, Dir::L(dist)) => (dist, 0, Face::E),
            (Face::E, Dir::R(dist)) | (Face::W, Dir::L(dist)) => (0, dist, Face::S),
            (Face::W, Dir::R(dist)) | (Face::E, Dir::L(dist)) => (0, -dist, Face::N),
        };

        x += dx;
        y += dy;

        d = nd;
    }

    x.abs() + y.abs()
}

fn puzzle_two(input: &str) -> isize {
    let instructions = parse(input);
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut d = Face::N;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    for i in instructions {
        let (dx, dy, nd) = match (d, i) {
            (Face::S, Dir::R(dist)) | (Face::N, Dir::L(dist)) => (-dist, 0, Face::W),
            (Face::N, Dir::R(dist)) | (Face::S, Dir::L(dist)) => (dist, 0, Face::E),
            (Face::E, Dir::R(dist)) | (Face::W, Dir::L(dist)) => (0, dist, Face::S),
            (Face::W, Dir::R(dist)) | (Face::E, Dir::L(dist)) => (0, -dist, Face::N),
        };

        let x_range = if dx < 0 {
            (x + dx..=x).rev().collect::<Vec<_>>()
        } else {
            (x..=x + dx).collect::<Vec<_>>()
        };

        let y_range = if dy < 0 {
            (y + dy..=y).rev().collect::<Vec<_>>()
        } else {
            (y..=y + dy).collect::<Vec<_>>()
        };

        let mut first = true;
        for nx in &x_range {
            for ny in &y_range {
                if visited.contains(&(*nx, *ny)) && !first {
                    return nx.abs() + ny.abs();
                }
                if visited.contains(&(*nx, *ny)) && first {
                    first = false;
                }
                x = *nx;
                y = *ny;
                visited.insert((*nx, *ny));
            }
        }

        d = nd;
    }

    x.abs() + y.abs()
}

#[cfg(test)]
mod tests {
    use crate::{puzzle_one, puzzle_two};

    #[test]
    fn test_puzzle_one() {
        let actual = puzzle_one(r"R2, L3");
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = puzzle_two(r"R8, R4, R4, R8");
        assert_eq!(actual, 4);
    }
}
