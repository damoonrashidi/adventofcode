use std::{cmp::Ordering, collections::HashMap, str::Chars};

fn main() {
    let input = include_str!("../inputs/2023/7.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let mut hands = parse(input);

    hands.sort_by(|a, b| {
        let a_value = get_value(&a.0);
        let b_value = get_value(&b.0);
        match a_value.cmp(&b_value) {
            Ordering::Equal => highest_value_card(&a.0, &b.0),
            x => x,
        }
    });

    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * bid)
        .sum()
}

fn puzzle_two(input: &str) -> usize {
    let mut hands = parse(input);

    hands.sort_by(|a, b| {
        let a_value = best_possible_value(&a.0);
        let b_value = best_possible_value(&b.0);
        match a_value.cmp(&b_value) {
            Ordering::Equal => highest_value_card(&a.0, &b.0),
            x => x,
        }
    });

    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * bid)
        .sum()
}

fn parse(input: &str) -> Vec<(Chars, usize)> {
    input
        .lines()
        .map(|line| {
            let (s, i) = line.split_at(6);
            (s.trim().chars(), i.parse::<usize>().unwrap())
        })
        .collect()
}

fn highest_value_card(a: &Chars, b: &Chars) -> Ordering {
    let ranking = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ]
    .into_iter();

    let a: Vec<char> = a.clone().collect();
    let b: Vec<char> = b.clone().collect();

    let mut i = 0;
    while i < a.len() {
        if a[i] == b[i] {
            i += 1;
            continue;
        }

        let a_pos = ranking.clone().position(|c| c == a[i]).unwrap();
        let b_pos = ranking.clone().position(|c| c == b[i]).unwrap();
        if a_pos < b_pos {
            return Ordering::Greater;
        }
        return Ordering::Less;
    }
    Ordering::Equal
}

fn best_possible_value(hand: &Chars) -> usize {
    // if no jokers
    let js = hand.clone().filter(|c| c == &'J').count();
    if js == 0 || js == 5 {
        return get_value(hand);
    }

    // duplicate highest count
    let mut counts: HashMap<char, usize> = HashMap::new();
    hand.clone().filter(|c| c != &'J').for_each(|c| {
        let count = counts.get(&c).unwrap_or(&0);
        counts.insert(c, count + 1);
    });

    let mut without_js: Vec<char> = hand.clone().filter(|c| c != &'J').collect();

    let (most_common_c, _) = counts
        .iter()
        .reduce(|(hc, hi_count), (c, count)| {
            if count > hi_count {
                (c, count)
            } else {
                (hc, hi_count)
            }
        })
        .unwrap();

    for _ in 0..js {
        without_js.push(*most_common_c);
    }

    let new_hand: String = without_js.into_iter().collect();

    get_value(&new_hand.chars())
}

fn get_value(hand: &Chars) -> usize {
    let mut map = HashMap::<char, usize>::new();

    for c in hand.clone() {
        let v = map.get(&c).unwrap_or(&0);
        map.insert(c, v + 1);
    }

    let values: Vec<&usize> = map.values().collect();
    let fives = values.contains(&&5);
    let fours = values.contains(&&4);
    let trips = values.contains(&&3);
    let doubles = map
        .values()
        .filter(|x| **x == 2)
        .collect::<Vec<&usize>>()
        .len();

    if fives {
        6
    } else if fours {
        5
    } else if trips && doubles == 1 {
        4
    } else if trips {
        3
    } else if doubles > 0 {
        doubles
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::{puzzle_one, puzzle_two};

    #[test]
    fn test() {
        let actual = puzzle_one(
            r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
",
        );
        assert_eq!(actual, 6440);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = puzzle_two(
            r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
",
        );
        assert_eq!(actual, 5905);
    }
}
