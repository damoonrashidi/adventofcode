fn main() {
    let input = include_str!("../../inputs/2023/17.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let _map = parse(input);
    0
}

fn puzzle_two(input: &str) -> usize {
    let _map = parse(input);
    0
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .enumerate()
        .map(|(_y, line)| {
            line.chars()
                .enumerate()
                .map(|(_x, c)| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{puzzle_one, puzzle_two};

    #[test]
    fn test_puzzle_one() {
        let actual = puzzle_one(
            r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        );
        assert_eq!(actual, 102);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
