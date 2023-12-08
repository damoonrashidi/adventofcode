use std::collections::HashMap;

fn main() {
    let input = include_str!("../inputs/2023/8.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let (instructions, map) = parse(input);

    let mut steps = 0;
    let mut i = 0;
    let mut place = "AAA";
    loop {
        if place == "ZZZ" {
            return steps;
        }
        let dir = instructions[i];
        let (l, r) = map.get(place).unwrap();
        place = if dir == 'L' { l } else { r };

        i = (i + 1) % instructions.len();
        steps += 1;
    }
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

fn parse(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let instructions = input
        .lines()
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .chars()
        .collect();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    input.lines().skip(2).for_each(|line| {
        let (src, rest) = line.split_once(" = ").unwrap();

        let (a, b) = rest
            .strip_prefix('(')
            .and_then(|c| c.strip_suffix(')'))
            .and_then(|c| c.split_once(", "))
            .unwrap();

        map.insert(src, (a, b));
    });

    (instructions, map)
}

#[cfg(test)]
mod tests {
    use crate::puzzle_one;

    #[test]
    fn test_puzzle_one() {
        let actual = puzzle_one(
            r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(actual, 2);

        let actual2 = puzzle_one(
            r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(actual2, 6);
    }

    // #[test]
    // fn test_puzzle_two() {
    //     let actual = puzzle_two(
    //         r"RL

    //     AAA = (BBB, CCC)
    //     BBB = (DDD, EEE)
    //     CCC = (ZZZ, GGG)
    //     DDD = (DDD, DDD)
    //     EEE = (EEE, EEE)
    //     GGG = (GGG, GGG)
    //     ZZZ = (ZZZ, ZZZ)",
    //     );
    //     assert_eq!(actual, 0);
    // }
}
