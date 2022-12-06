use std::{collections::HashSet, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).unwrap();

    println!("{}", day1(file));
}

fn day1(input: String) -> usize {
    for i in 0..(input.len() - 3) {
        let slice: HashSet<char> = HashSet::from(
            input[i..i + 4]
                .chars()
                .into_iter()
                .collect::<HashSet<char>>(),
        );

        if slice.len() == 4 {
            return i + 4;
        };
    }
    panic!("No marker found");
}

#[cfg(test)]
mod tests {
    use crate::day1;

    #[test]
    fn day_6() {
        for (case, expected) in [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ] {
            assert_eq!(day1(case.into()), expected);
        }
    }
}
