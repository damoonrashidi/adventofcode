use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/2016/3.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|w| w.parse::<usize>().unwrap())
                .sorted()
                .collect::<Vec<_>>()
        })
        .filter(|tri| tri[0] + tri[1] > tri[2])
        .count()
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use crate::{puzzle_one, puzzle_two};

    #[test]
    fn test_puzzle_one() {
        let actual = puzzle_one(r"");
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
