fn main() {
    let input = parse(include_str!("../../inputs/2023/9.txt").trim());
    println!("puzzle one: {}", puzzle_one(&input));
    println!("puzzle two: {}", puzzle_two(&input));
}

fn puzzle_one(input: &[Vec<isize>]) -> isize {
    input
        .iter()
        .map(|row| {
            make_all_zeroes(vec![row.clone()])
                .iter()
                .filter_map(|row| row.last())
                .rev()
                .sum::<isize>()
        })
        .sum()
}

fn puzzle_two(input: &[Vec<isize>]) -> isize {
    input
        .iter()
        .map(|row| {
            make_all_zeroes(vec![row.clone()])
                .iter()
                .filter_map(|row| row.first())
                .rev()
                .fold(0, |a, b| b - a)
        })
        .sum()
}

fn make_all_zeroes(mut tail: Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    let last = tail.last().unwrap();
    if last.iter().all(|element| element == &0) {
        return tail;
    }
    let mut steps = vec![];
    for i in 0..last.len() - 1 {
        steps.push(last[i + 1] - last[i]);
    }
    tail.push(steps);
    make_all_zeroes(tail)
}

fn parse(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|value| value.parse().ok())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse, puzzle_one, puzzle_two};

    #[test]
    fn test_puzzle_one() {
        let input = parse(
            r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        let actual = puzzle_one(&input);
        assert_eq!(actual, 114);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = puzzle_two(&parse(
            r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        ));
        assert_eq!(actual, 2);
    }
}
