use std::collections::HashSet;

use rayon::prelude::*;

fn main() {
    let input = include_str!("../../inputs/2024/6.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn solve(walls: &[Vec<bool>], (mut x, mut y): (usize, usize)) -> Option<HashSet<(usize, usize)>> {
    let mut dir = Dir::Up;
    let width = walls[0].len();
    let height = walls.len();
    let mut seen: HashSet<(usize, usize)> = HashSet::from([(x, y)]);
    let mut seen_dir: HashSet<(usize, usize, Dir)> = HashSet::from([(x, y, dir)]);

    while (0..width).contains(&x) && (0..height).contains(&y) {
        let (dx, dy) = match dir {
            Dir::Up => (0, -1),
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
        };
        let (Some(new_x), Some(new_y)) = (x.checked_add_signed(dx), y.checked_add_signed(dy))
        else {
            break;
        };

        if !(0..width).contains(&new_x) || !(0..height).contains(&new_y) {
            break;
        }

        if walls[new_y][new_x] {
            dir = match dir {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
            };
            continue;
        }

        y = new_y;
        x = new_x;

        if seen_dir.contains(&(new_x, new_y, dir)) {
            return None;
        }

        seen_dir.insert((x, y, dir));

        seen.insert((x, y));
    }

    Some(seen)
}

fn parse(input: &str) -> (Vec<Vec<bool>>, (usize, usize)) {
    let walls: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let start_pos = input.replacen('\n', "", input.len()).find('^').unwrap();
    let width = walls[0].len();
    let height = walls.len();

    (walls, (start_pos % width, start_pos / height))
}

fn puzzle_one(input: &str) -> usize {
    let (walls, (x, y)) = parse(input);
    solve(&walls, (x, y)).unwrap().len()
}

fn puzzle_two(input: &str) -> usize {
    let (walls, (x, y)) = parse(input);

    solve(&walls, (x, y))
        .unwrap()
        .into_par_iter()
        .map(|(nx, ny)| {
            let mut walls = walls.clone();
            walls[ny][nx] = true;
            solve(&walls, (x, y))
        })
        .filter(Option::is_none)
        .count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );
        assert_eq!(actual, 41);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(
            r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );
        assert_eq!(actual, 6);
    }
}
