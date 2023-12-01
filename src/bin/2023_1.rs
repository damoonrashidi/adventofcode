use std::str::Lines;

fn main() {
    println!("part one: {}", part_one());
    println!(
        "part two: {}",
        part_two(include_str!("../inputs/2023_1.txt").lines())
    );
}

fn part_one() -> u32 {
    include_str!("../inputs/2023_1.txt")
        .lines()
        .map(|line| {
            let x = line
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<Vec<char>>();

            let first = x.first().and_then(|x| x.to_digit(10));
            let second = x.last().and_then(|x| x.to_digit(10));

            if let (Some(first), Some(second)) = (first, second) {
                return first * 10 + second;
            }
            0
        })
        .sum()
}

fn part_two(lines: Lines) -> u32 {
    lines
        .map(|line| {
            let valids = [
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
                "six", "seven", "eight", "nine",
            ];

            let first_occurances = valids.into_iter().filter_map(|valid| {
                if let Some(index) = line.find(valid) {
                    return Some((index, valid));
                }
                None
            });

            let last_occurrances = valids.into_iter().filter_map(|valid| {
                if let Some(index) = line.rfind(valid) {
                    return Some((index, valid));
                }
                None
            });

            let mut first_i = 500;
            let mut first_word = "";
            for (occurance_i, word) in first_occurances {
                if occurance_i <= first_i {
                    first_i = occurance_i;
                    first_word = word;
                }
            }

            let mut last_i = 0;
            let mut last_word = "";
            for (occurance_i, word) in last_occurrances {
                if occurance_i >= last_i {
                    last_i = occurance_i;
                    last_word = word;
                }
            }

            println!("{line} {first_word} {last_word}");

            str_to_digit(first_word) * 10 + str_to_digit(last_word)
        })
        .sum()
}

#[allow(unused)]
fn str_to_digit(digit: &str) -> u32 {
    match digit {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => 0,
    }
}
