use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/2015/13.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn parse(input: &str) -> (HashMap<&str, HashMap<&str, isize>>, HashSet<&str>) {
    let mut map = HashMap::new();
    let mut list = HashSet::new();

    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let person = words[0];
        let sign = if words[2] == "lose" { -1 } else { 1 };
        let amount = words[3].parse::<isize>().unwrap() * sign;
        let last_word = words.last().unwrap();
        let neighbor = &last_word[0..last_word.len() - 1];

        list.insert(person);

        if !map.contains_key(person) {
            map.insert(person, HashMap::new());
        }

        let p = map.get_mut(person).unwrap();
        p.insert(neighbor, amount);
    }

    (map, list)
}

fn puzzle_one(input: &str) -> isize {
    let (map, set) = parse(input);
    let k = set.len();
    let perms: Vec<Vec<&str>> = set.into_iter().permutations(k).collect();
    let mut cost = 0;

    for permutation in perms {
        let mut curr = 0;
        for (i, person) in permutation.iter().enumerate() {
            let left = if i == 0 {
                permutation.last().unwrap()
            } else {
                permutation[i - 1]
            };

            let right = if i == permutation.len() - 1 {
                permutation[0]
            } else {
                permutation[i + 1]
            };

            let left_cost = map.get(person).unwrap().get(left).unwrap();
            let right_cost = map.get(person).unwrap().get(right).unwrap();

            curr += left_cost + right_cost;
        }
        if curr > cost {
            cost = curr;
        }
    }

    cost
}

fn puzzle_two(input: &str) -> isize {
    let (mut map, mut set) = parse(input);
    set.insert("Me");
    for (_, sub) in &mut map.iter_mut() {
        sub.insert("Me", 0);
    }

    let mut my_map = HashMap::new();

    for person in &set {
        my_map.insert(*person, 0);
    }
    map.insert("Me", my_map);

    let k = set.len();
    let perms: Vec<Vec<&str>> = set.into_iter().permutations(k).collect();
    let mut cost = 0;

    for permutation in perms {
        let mut curr = 0;
        for (i, person) in permutation.iter().enumerate() {
            let left = if i == 0 {
                permutation.last().unwrap()
            } else {
                permutation[i - 1]
            };

            let right = if i == permutation.len() - 1 {
                permutation[0]
            } else {
                permutation[i + 1]
            };

            let left_cost = map.get(person).unwrap().get(left).unwrap();
            let right_cost = map.get(person).unwrap().get(right).unwrap();

            curr += left_cost + right_cost;
        }
        if curr > cost {
            cost = curr;
        }
    }

    cost
}

#[cfg(test)]
mod tests {
    use crate::{puzzle_one, puzzle_two};

    #[test]
    fn test_puzzle_one() {
        let actual = puzzle_one(
            r"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.",
        );
        assert_eq!(actual, 330);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
