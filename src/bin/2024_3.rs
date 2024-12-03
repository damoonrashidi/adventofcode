fn main() {
    let input = include_str!("../../inputs/2024/3.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let options = input.split("mul(").collect::<Vec<_>>();
    let mut total = 0;
    for option in &options {
        let scan = option.chars().take_while(|c| c != &')').collect::<String>();
        if let Some((a, b)) = scan.split_once(',') {
            if let (Ok(a), Ok(b)) = (a.parse::<usize>(), b.parse::<usize>()) {
                total += a * b;
            }
        }
    }

    total
}

fn puzzle_two(input: &str) -> usize {
    let options = input.split("mul(").collect::<Vec<_>>();
    let mut total = 0;
    let mut ignore = false;
    for (i, option) in options.iter().enumerate() {
        if let Some(prev) = options.get(i.saturating_sub(1)) {
            let do_pos = prev.find("do()");
            let dont_pos = prev.find("don't()");

            match (do_pos, dont_pos) {
                (None, None) => {}
                (None, Some(_)) => ignore = true,
                (Some(_), None) => ignore = false,
                (Some(a), Some(b)) => ignore = a < b,
            }
        }
        if ignore {
            continue;
        }
        let scan = option.chars().take_while(|c| c != &')').collect::<String>();
        if let Some((a, b)) = scan.split_once(',') {
            if let (Ok(a), Ok(b)) = (a.parse::<usize>(), b.parse::<usize>()) {
                total += a * b;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );
        assert_eq!(actual, 161);
    }

    #[test]
    fn test_puzzle_two() {
        assert_eq!(
            crate::puzzle_two(
                r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            ),
            48
        );

        assert_eq!(
            crate::puzzle_two(
                r"xmul(2,4)&mul[3,7]!^don't()_do()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            ),
            161
        );
    }
}
