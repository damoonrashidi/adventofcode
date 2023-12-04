use std::collections::HashSet;

fn main() {
    let input = include_str!("../inputs/2023_4.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    input
        .lines()
        .map(parse_row)
        .map(|(winning, mine)| {
            let matching = winning.intersection(&mine).count();
            2usize.pow((matching.saturating_sub(1)).try_into().unwrap())
        })
        .sum()
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

fn parse_row(line: &str) -> (HashSet<usize>, HashSet<usize>) {
    let (a, b) = line.split_once(" | ").unwrap();
    let (_, list) = a.split_once(": ").unwrap();

    let mine_numbers = b
        .split_whitespace()
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect();

    let winning_numbers = list
        .split_whitespace()
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect();

    (winning_numbers, mine_numbers)
}

#[cfg(test)]
mod tests {
    use crate::puzzle_one;

    #[test]
    fn test() {
        let actual = puzzle_one(
            r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(actual, 13);
    }
}
