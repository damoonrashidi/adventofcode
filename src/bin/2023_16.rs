use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/2023/16.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Beam {
    pub(self) x: usize,
    pub(self) y: usize,
    pub(self) dir: Direction,
}

impl Beam {
    pub(self) fn new(x: usize, y: usize, dir: Direction) -> Self {
        Self { x, y, dir }
    }
}

fn puzzle_one(input: &str) -> usize {
    let map = parse(input);
    let beam = Beam::new(0, 0, Direction::Right);
    let mut beams = vec![beam];
    solve(&mut beams, &map, HashSet::new())
}

fn puzzle_two(input: &str) -> usize {
    let map = parse(input);
    let mut max_value = 0;

    for i in 0..map[0].len() {
        let beam = Beam::new(i, 0, Direction::Down);
        let mut beams = vec![beam];
        let v = solve(&mut beams, &map, HashSet::new());
        if v > max_value {
            max_value = v;
        }
    }

    for i in 0..map[0].len() {
        let beam = Beam::new(i, map.len() - 1, Direction::Up);
        let mut beams = vec![beam];
        let v = solve(&mut beams, &map, HashSet::new());
        if v > max_value {
            max_value = v;
        }
    }

    for i in 0..map.len() {
        let beam = Beam::new(0, i, Direction::Up);
        let mut beams = vec![beam];
        let v = solve(&mut beams, &map, HashSet::new());
        if v > max_value {
            max_value = v;
        }
    }

    for i in 0..map.len() {
        let beam = Beam::new(map[0].len() - 1, i, Direction::Up);
        let mut beams = vec![beam];
        let v = solve(&mut beams, &map, HashSet::new());
        if v > max_value {
            max_value = v;
        }
    }

    max_value
}

#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn solve(beams: &mut Vec<Beam>, map: &[Vec<char>], mut seen: HashSet<(usize, usize)>) -> usize {
    let mut visits: HashSet<(usize, usize, Direction)> = HashSet::new();

    while !beams.is_empty() {
        for i in (0..beams.len()).rev() {
            let c = map[beams[i].y][beams[i].x];
            seen.insert((beams[i].x, beams[i].y));
            visits.insert((beams[i].x, beams[i].y, beams[i].dir));

            let splits = get_splits(&beams[i], map);
            if !splits.is_empty() {
                beams.swap_remove(i);
                beams.extend(splits);
                continue;
            }

            if let Some(dir) = get_dir(beams[i].dir, c) {
                let new_pos = get_new_pos(dir, (beams[i].x, beams[i].y), map);
                if new_pos.is_none() {
                    beams.swap_remove(i);
                    continue;
                }
                let (new_x, new_y) = new_pos.unwrap();

                if visits.contains(&(new_x, new_y, dir)) {
                    beams.swap_remove(i);
                    continue;
                }

                beams[i].x = new_x;
                beams[i].y = new_y;
                beams[i].dir = dir;
                visits.insert((beams[i].x, beams[i].y, beams[i].dir));
            }
        }
    }

    seen.len()
}

fn get_dir(src: Direction, c: char) -> Option<Direction> {
    match (c, src) {
        ('/', Direction::Left) | ('\\', Direction::Right) => Some(Direction::Down),
        ('/', Direction::Right) | ('\\', Direction::Left) => Some(Direction::Up),
        ('/', Direction::Down) | ('\\', Direction::Up) => Some(Direction::Left),
        ('\\', Direction::Down) | ('/', Direction::Up) => Some(Direction::Right),
        ('.', src) => Some(src),
        ('-', Direction::Left | Direction::Right) | ('|', Direction::Down | Direction::Up) => {
            Some(src)
        }
        _ => None,
    }
}

fn get_splits(beam: &Beam, map: &[Vec<char>]) -> Vec<Beam> {
    let c = map[beam.y][beam.x];

    let mut beams = vec![];

    if (beam.dir == Direction::Up || beam.dir == Direction::Down) && c == '-' {
        if let Some((x, y)) = get_new_pos(Direction::Left, (beam.x, beam.y), map) {
            beams.push(Beam::new(x, y, Direction::Left));
        }

        if let Some((x, y)) = get_new_pos(Direction::Right, (beam.x, beam.y), map) {
            beams.push(Beam::new(x, y, Direction::Right));
        }
    }

    if (beam.dir == Direction::Left || beam.dir == Direction::Right) && c == '|' {
        if let Some((x, y)) = get_new_pos(Direction::Up, (beam.x, beam.y), map) {
            beams.push(Beam::new(x, y, Direction::Up));
        }
        if let Some((x, y)) = get_new_pos(Direction::Down, (beam.x, beam.y), map) {
            beams.push(Beam::new(x, y, Direction::Down));
        }
    }

    beams
}

#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn get_new_pos(
    direction: Direction,
    point: (usize, usize),
    map: &[Vec<char>],
) -> Option<(usize, usize)> {
    let (x, y) = match direction {
        Direction::Up => (0, -1),
        Direction::Left => (-1, 0),
        Direction::Down => (0, 1),
        Direction::Right => (1, 0),
    };

    let new_x = point.0 as isize + x;
    let new_y = point.1 as isize + y;

    if new_x >= 0 && new_x < map[0].len() as isize && new_y >= 0 && new_y < map.len() as isize {
        return Some((new_x as usize, new_y as usize));
    }
    None
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use crate::puzzle_two;

    #[test]
    fn test_puzzle_one() {
        let actual = puzzle_two(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        );
        assert_eq!(actual, 51);
    }
}
