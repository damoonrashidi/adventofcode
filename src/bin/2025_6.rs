fn main() {
    let input = include_str!("../../inputs/2025/6.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let numbers: Vec<Vec<_>> = input
        .lines()
        .take_while(|line| !line.contains('+'))
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let ops = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();

    let mut totals = numbers[0].clone();

    for col in 0..numbers[0].len() {
        for row in 1..numbers.len() {
            match ops[col] {
                "+" => {
                    totals[col] += numbers[row][col];
                }
                "*" => {
                    totals[col] *= numbers[row][col];
                }
                _ => {}
            }
        }
    }

    totals.iter().sum()
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ",
        );
        assert_eq!(actual, 4_277_556);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
