use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../inputs/2024/5.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn is_correct(ordering: &HashMap<usize, HashSet<usize>>, list: &[usize]) -> bool {
    (0..list.len() - 1).all(|i| {
        ordering
            .get(&list[i + 1])
            .map_or(true, |befores| !befores.contains(&list[i]))
    })
}

fn fix(ordering: &HashMap<usize, HashSet<usize>>, list: &[usize]) -> Vec<usize> {
    let mut out = list.to_vec();

    for i in 0..out.len() {
        for j in i + 1..out.len() {
            if let Some(before) = ordering.get(&out[j]) {
                if before.contains(&out[i]) {
                    out.swap(i, j);
                }
            }
        }
    }

    out
}

fn parse(input: &str) -> (HashMap<usize, HashSet<usize>>, Vec<Vec<usize>>) {
    let mut ordering: HashMap<usize, HashSet<usize>> = HashMap::new();
    let (ordering_str, produce) = input.split_once("\n\n").unwrap();
    ordering_str.lines().for_each(|line| {
        let (key, after) = line.split_once('|').unwrap();
        let key = key.parse::<usize>().unwrap();
        let after = after.parse::<usize>().unwrap();

        ordering
            .entry(key)
            .or_insert(HashSet::from([after]))
            .insert(after);
    });

    let updates = produce
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (ordering, updates)
}

fn puzzle_one(input: &str) -> usize {
    let (ordering, updates) = parse(input);

    updates
        .iter()
        .filter(|update| is_correct(&ordering, update))
        .map(|update| update[update.len() / 2])
        .sum::<usize>()
}

fn puzzle_two(input: &str) -> usize {
    let (ordering, updates) = parse(input);

    updates
        .iter()
        .filter(|u| !is_correct(&ordering, u))
        .map(|u| fix(&ordering, u))
        .map(|u| u[u.len() / 2])
        .sum()
}
