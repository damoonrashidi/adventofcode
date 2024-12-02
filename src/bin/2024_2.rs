fn main() {
    let input = include_str!("../../inputs/2024/2.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(data: &[u32]) -> bool {
    let increasing = data[0] < data[1];
    for i in 0..data.len() - 1 {
        if (increasing && data[i] > data[i + 1]) || (!increasing && data[i] < data[i + 1]) {
            return false;
        }
        let diff = data[i].abs_diff(data[i + 1]);
        if !(1..=3).contains(&diff) {
            return false;
        }
    }
    true
}

fn puzzle_one(input: &str) -> u32 {
    parse(input)
        .iter()
        .filter(|data| is_safe(data))
        .count()
        .try_into()
        .unwrap()
}

fn puzzle_two(input: &str) -> u32 {
    parse(input)
        .iter()
        .filter(|data| {
            if is_safe(data) {
                return true;
            }

            for i in 0..data.len() {
                let mut new = Vec::from(&data[0..i]);
                new.extend_from_slice(&data[i + 1..data.len()]);
                if is_safe(&new) {
                    return true;
                }
            }

            false
        })
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(
            r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(actual, 4);
    }
}
