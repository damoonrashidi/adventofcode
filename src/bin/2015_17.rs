fn main() {
    let input = include_str!("../../inputs/2015/17.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    input.len()
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r"20
15
10
5
5",
        );
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
