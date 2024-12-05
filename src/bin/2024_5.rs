use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/2024/5.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let mut ordering: HashMap<usize, Vec<usize>> = HashMap::new();
    let (ordering_str, produce) = input.split_once("\n\n").unwrap();
    ordering_str.lines().for_each(|line| {
        let (key, after) = line.split_once('|').unwrap();
        let key = key.parse::<usize>().unwrap();
        let after = after.parse::<usize>().unwrap();

        ordering.entry(key).or_insert(vec![after]).push(after);
    });

    let updates = produce
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut total = 0;

    for update in &updates {
        let mut correct = true;
        for i in 0..update.len() - 1 {
            if let Some(list) = &ordering.get(&update[i + 1]) {
                if list.contains(&update[i]) {
                    correct = false;
                    break;
                }
            }
        }
        if correct {
            total += update[update.len() / 2];
        }
    }

    total
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(actual, 143);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
