fn main() {
    let input = include_str!("../../inputs/2015/12.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> isize {
    let mut current: Vec<char> = vec![];
    let mut sum: isize = 0;

    for c in input.chars() {
        if c.is_numeric() || c == '-' {
            current.push(c);
        } else if !current.is_empty() {
            if let Ok(parsed) = current.iter().collect::<String>().parse::<isize>() {
                sum += parsed;
            }
            current = vec![];
        }
    }

    sum
}

fn puzzle_two(_input: &str) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use crate::{puzzle_one, puzzle_two};

    #[test]
    fn test_puzzle_one() {
        assert_eq!(puzzle_one("[1,2,3]"), 6);
        assert_eq!(puzzle_one(r#"{"a":2,"b":4}"#), 6);
        assert_eq!(puzzle_one(r"{}"), 0);
        assert_eq!(puzzle_one("[-1,5]"), 4);
        assert_eq!(puzzle_one("[[[3]]]"), 3);
    }

    #[test]
    fn test_puzzle_two() {
        assert_eq!(puzzle_two("[1,2,3]"), 6);
        assert_eq!(puzzle_two(r#"{"a":"red","b":4}"#), 0);
        assert_eq!(puzzle_two(r"{}"), 0);
        assert_eq!(puzzle_two("[-1,5]"), 4);
        assert_eq!(puzzle_two(r#"[[[3], {"a": "red"}]]"#), 3);
    }
}
