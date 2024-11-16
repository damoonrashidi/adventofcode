fn main() {
    let input = include_str!("../../inputs/2015/11.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two("hepxxzzz"));
}

fn is_valid(password: &str) -> bool {
    let mut overlapping_count = 0;
    let mut has_incrementing_triple = false;
    let chars = password.chars().collect::<Vec<char>>();
    for (i, letter) in chars.iter().enumerate() {
        if ['i', 'o', 'l'].contains(letter) {
            return false;
        }

        if i >= 2 {
            if let Some(prev) = chars.get(i - 2) {
                if prev == letter {
                    overlapping_count -= 1;
                }
            }
        }

        if let Some(next) = chars.get(i + 1) {
            if next == letter {
                overlapping_count += 1;
            }

            if let Some(even_more_next) = chars.get(i + 2) {
                if *letter as u8 + 1 == *next as u8 && *next as u8 + 1 == *even_more_next as u8 {
                    has_incrementing_triple = true;
                }
            }
        }
    }

    overlapping_count >= 2 && has_incrementing_triple
}

fn increment(password: &str) -> String {
    let mut chars: Vec<char> = password.chars().rev().collect();
    for i in 0..chars.len() {
        if chars[i] == 'z' {
            chars[i] = 'a';
        } else {
            chars[i] = ((chars[i] as u8) + 1) as char;
            return chars.iter().rev().collect();
        }
    }
    chars.iter().rev().collect()
}

fn puzzle_one(input: &str) -> String {
    let mut new_password = input.to_string();
    while !is_valid(&new_password) {
        new_password = increment(&new_password);
    }

    new_password.to_string()
}

fn puzzle_two(input: &str) -> String {
    let new = puzzle_one(input);
    puzzle_one(&new)
}

#[cfg(test)]
mod tests {
    use crate::is_valid;

    #[test]
    fn test_is_valid() {
        assert!(!is_valid("hijklmmn"));
        assert!(!is_valid("abbceffg"));
        assert!(!is_valid("abcdeggg"));
        assert!(!is_valid("abbcegjk"));
        assert!(is_valid("abccegjj"));
        assert!(is_valid("abcdffaa"));
        assert!(is_valid("ghjaabcc"));
    }

    #[test]
    fn test_puzzle_one() {
        assert_eq!(crate::puzzle_one("abcdefgh"), "abcdffaa");
        assert_eq!(crate::puzzle_one("ghijklmn"), "ghjaabcc");
    }
}
