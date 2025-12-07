use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../../inputs/2025/5.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let ranges = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once('-').unwrap())
        .map(|(a, b)| a.parse::<usize>().unwrap()..=b.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.parse::<usize>().unwrap())
        .map(|i| {
            for r in &ranges {
                if r.contains(&i) {
                    return 1;
                }
            }

            0
        })
        .sum()
}

fn puzzle_two(input: &str) -> usize {
    let ranges = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once('-').unwrap())
        .map(|(a, b)| a.parse::<usize>().unwrap()..=b.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut unioned_ranges: Vec<RangeInclusive<usize>> = vec![];

    for range in ranges {
        for mut union_range in unioned_ranges {
            if should_be_unioned(a, b) {
                union(range, union_range);
            }
        }
    }

    0
}

fn should_be_unioned(a: RangeInclusive<usize>, b: RangeInclusive<usize>) -> bool {
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r"3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        );
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
