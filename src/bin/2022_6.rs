use std::{collections::HashSet, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).unwrap();

    println!("{}", puzzle_one(&file));
    println!("{}", puzzle_two(&file));
}

fn puzzle_one(input: &String) -> usize {
    for i in 0..(input.len() - 3) {
        let slice: HashSet<char> = input[i..i + 4]
                .chars()
                .collect::<HashSet<char>>();

        if slice.len() == 4 {
            return i + 4;
        };
    }
    panic!("No marker found");
}

fn puzzle_two(input: &String) -> usize {
    for i in 0..(input.len() - 3) {
        let slice: HashSet<char> = input[i..i + 14]
                .chars()
                .collect::<HashSet<char>>();

        if slice.len() == 14 {
            return i + 14;
        };
    }
    panic!("No marker found");
}

#[cfg(test)]
mod tests {
    use crate::{puzzle_one, puzzle_two};

    #[test]
    fn day_6() {
        for (case, expected) in [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ] {
            assert_eq!(puzzle_one(&case.into()), expected);
        }
    }

    #[test]
    fn day_6_part_2() {
        for (case, expected) in [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ] {
            assert_eq!(puzzle_two(&case.into()), expected);
        }
    }
}
