fn main() {
    let input = include_str!("../../inputs/2025/3.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> u32 {
    let l = input.chars().take_while(|c| !c.is_whitespace()).count();

    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .map(|line| {
            let line = line.collect::<Vec<_>>();
            let mut head = line[0];
            let mut tail = line[1];
            for i in 0..l - 1 {
                if line[i] > head {
                    head = line[i];
                    tail = line[i + 1];
                } else if line[i + 1] > tail {
                    tail = line[i + 1];
                }
            }

            println!("{head}{tail}");

            head * 10 + tail
        })
        .sum::<u32>()
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r"987654321111111
811111111111119
234234234234278
818181911112111",
        );
        assert_eq!(actual, 357);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
