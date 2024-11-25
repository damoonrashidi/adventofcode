fn main() {
    let input = include_str!("../../inputs/2015/15.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn parse(input: &str) -> Vec<[isize; 5]> {
    let mut stuff = vec![];

    for line in input.lines() {
        let parsed: Vec<isize> = line
            .replace(',', " ")
            .split_whitespace()
            .filter_map(|word| word.parse::<isize>().ok())
            .collect();
        stuff.push([parsed[0], parsed[1], parsed[2], parsed[3], parsed[4]]);
    }

    stuff
}

fn split(amount: usize, n: usize) -> Vec<Vec<usize>> {
    let mut results = Vec::new();

    if n == 1 {
        return vec![vec![amount]];
    }

    for i in 0..=amount {
        let mut sub_splits = split(amount - i, n - 1);
        for split in &mut sub_splits {
            split.insert(0, i);
        }
        results.extend(sub_splits);
    }

    results
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn puzzle_one(input: &str) -> isize {
    let ingredients = parse(input);

    let splits = split(100, ingredients.len());

    let mut best = 0;

    for split in splits {
        let [mut cap, mut dur, mut fla, mut tex, _] = [0; 5];
        for (i, ingredient) in ingredients.iter().enumerate() {
            cap += ingredient[0] * split[i] as isize;
            dur += ingredient[1] * split[i] as isize;
            fla += ingredient[2] * split[i] as isize;
            tex += ingredient[3] * split[i] as isize;
        }
        let curr = cap.max(0) * dur.max(0) * fla.max(0) * tex.max(0);
        if curr > best {
            best = curr;
        }
    }

    best
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn puzzle_two(input: &str) -> isize {
    let ingredients = parse(input);

    let splits = split(100, ingredients.len());

    let mut best = 0;

    for split in splits {
        let [mut cap, mut dur, mut fla, mut tex, mut calories] = [0; 5];
        for (i, ingredient) in ingredients.iter().enumerate() {
            cap += ingredient[0] * split[i] as isize;
            dur += ingredient[1] * split[i] as isize;
            fla += ingredient[2] * split[i] as isize;
            tex += ingredient[3] * split[i] as isize;
            calories += ingredient[4] * split[i] as isize;
        }
        if calories == 500 {
            let curr = cap.max(0) * dur.max(0) * fla.max(0) * tex.max(0);
            if curr > best {
                best = curr;
            }
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use crate::{puzzle_one, puzzle_two};

    #[test]
    fn test_puzzle_one() {
        let actual = puzzle_one(
            r"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        );
        assert_eq!(actual, 62_842_880);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = puzzle_two(
            r"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        );
        assert_eq!(actual, 57_600_000);
    }
}
