use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../inputs/2023/4.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    input
        .lines()
        .map(parse_row)
        .map(|(_, winning, mine)| {
            let matching = winning.intersection(&mine).count();
            if matching == 0 {
                return 0;
            }
            let mut tot = 1;
            for _ in 1..matching {
                tot *= 2;
            }
            tot
        })
        .sum()
}

fn puzzle_two(input: &str) -> usize {
    let cards: Vec<(&str, HashSet<usize>, HashSet<usize>)> = input.lines().map(parse_row).collect();
    let mut clones: HashMap<&str, usize> = HashMap::new();

    let mut i = 0;
    while i < cards.len() {
        let (id, winning, mine) = &cards[i];
        let matches = winning.intersection(mine).count();
        if matches > 0 {
            let ids_to_add = cards[1 + i..=matches + i].iter().map(|(id, ..)| id);
            let add_count = clones.get(id).unwrap_or(&0) + 1;
            for id_to_add in ids_to_add {
                let old_count = clones.get(id_to_add).unwrap_or(&0);
                clones.insert(id_to_add, old_count + add_count);
            }
        }
        i += 1;
    }

    dbg!(&clones);

    cards.len() + clones.values().sum::<usize>()
}

fn parse_row(line: &str) -> (&str, HashSet<usize>, HashSet<usize>) {
    let (a, b) = line.split_once(" | ").unwrap();
    let (id, list) = a.split_once(": ").unwrap();

    let mine_numbers = b
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let winning_numbers = list
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    (id, winning_numbers, mine_numbers)
}

#[cfg(test)]
mod tests {
    use crate::puzzle_two;

    #[test]
    fn test() {
        let actual = puzzle_two(
            r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(actual, 31);
    }
}
