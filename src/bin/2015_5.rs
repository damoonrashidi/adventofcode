fn main() {
    let input = include_str!("../inputs/2015_5.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle one: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let is_nice = |str: &str| -> bool {
        let illegal_bigrams = ["ab", "cd", "pq", "xy"];
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let mut has_double = false;

        for (i, c) in str.chars().enumerate() {
            if Some(c) == str.chars().nth(i + 1) {
                has_double = true;
                break;
            }
        }

        let has_illegal_bigram = illegal_bigrams.iter().any(|bigram| str.contains(bigram));
        let vowel_count = str.chars().filter(|c| vowels.contains(c)).count();

        !has_illegal_bigram && vowel_count >= 3 && has_double
    };

    input.lines().filter(|line| is_nice(line)).count()
}

fn puzzle_two(input: &str) -> usize {
    let is_nice = |str: &str| -> bool {
        let mut has_spaced_single = false;
        let mut has_repeating_bigram = false;

        for (i, c) in str.chars().enumerate() {
            if Some(c) == str.chars().nth(i + 2) {
                has_spaced_single = true;
                break;
            }
        }

        for (i, c) in str.chars().enumerate() {
            if let Some(next) = str.chars().nth(i + 1) {
                let remainder = &str[i + 2..];
                let bigram = format!("{c}{next}");
                if remainder.contains(bigram.as_str()) {
                    has_repeating_bigram = true;
                    break;
                }
            }
        }

        has_spaced_single && has_repeating_bigram
    };

    input.lines().filter(|line| is_nice(line)).count()
}
