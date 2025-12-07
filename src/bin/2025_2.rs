fn main() {
    let input = include_str!("../../inputs/2025/2.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    input
        .split(',')
        .map(|i| i.split_once('-').unwrap())
        .map(|(a, b)| {
            let a = a.trim().parse::<usize>().unwrap();
            let b = b.trim().parse::<usize>().unwrap();

            (a, b)
        })
        .flat_map(|(first, last)| {
            let mut invalids = vec![];
            for x in first..=last {
                let f = format!("{x}");
                let d = f.len() / 2;
                let h1 = &f[0..d];
                let h2 = &f[d..];

                if h1 == h2 {
                    invalids.push(x);
                }
            }

            invalids
        })
        .sum::<usize>()
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224, 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(actual, 1_227_775_554);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
