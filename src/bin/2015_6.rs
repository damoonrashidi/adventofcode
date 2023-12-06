use regex::Regex;

fn main() {
    let input = include_str!("../inputs/2015/6.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle one: {}", puzzle_two(input));
}

#[derive(Debug)]
struct Coord(usize, usize);

#[derive(Debug)]
#[allow(unused)]
enum Instruction {
    On(Coord, Coord),
    Off(Coord, Coord),
    Toggle(Coord, Coord),
}

fn puzzle_one(input: &str) -> usize {
    let mut city = vec![vec![false; 1000]; 1000];

    input.trim().lines().for_each(|line| {
        let (from, to) = get_coords(line);

        (from.0..=to.0).for_each(|x| {
            (from.1..=to.1).for_each(|y| {
                city[x][y] = if line.contains("on") {
                    true
                } else if line.contains("off") {
                    false
                } else {
                    !city[x][y]
                };
            });
        });
    });

    city.into_iter()
        .map(|r| r.into_iter().filter(|x| *x).count())
        .sum()
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

fn get_coords(line: &str) -> (Coord, Coord) {
    let coord_regex = Regex::new(r"(?P<from>\d+),(?P<to>\d+)").unwrap();

    let mut caps = coord_regex.captures_iter(line);
    let (from_x, from_y) = caps
        .next()
        .map(|cap| {
            (
                cap.name("from").unwrap().as_str().parse::<usize>().unwrap(),
                cap.name("to").unwrap().as_str().parse::<usize>().unwrap(),
            )
        })
        .unwrap();

    let (to_x, to_y) = caps
        .next()
        .map(|cap| {
            (
                cap.name("from").unwrap().as_str().parse::<usize>().unwrap(),
                cap.name("to").unwrap().as_str().parse::<usize>().unwrap(),
            )
        })
        .unwrap();

    (Coord(from_x, from_y), Coord(to_x, to_y))
}

#[cfg(test)]
mod tests {
    use crate::puzzle_one;

    #[test]
    fn test() {
        let actual = puzzle_one("turn on 499,499 through 500,500");
        assert_eq!(actual, 4);
    }
}
