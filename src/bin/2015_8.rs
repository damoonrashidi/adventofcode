fn main() {
    let input = include_str!("../../inputs/2015/8.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let mut code = 0;
    let mut mem = 0;

    let reg = regex::Regex::new(r"\\x[a-f0-9]{2}").unwrap();
    for code_str in input.lines() {
        let replaced = code_str.replace("\\\\", " ").replace("\\\"", " ");
        let mem_str = reg.replace_all(&replaced, " ");
        code += code_str.len();
        mem += mem_str.len() - 2;
    }
    code - mem
}

fn puzzle_two(input: &str) -> usize {
    let mut code = 0;
    let mut mem = 0;

    let reg = regex::Regex::new(r"\\x[a-f0-9]{2}").unwrap();
    for code_str in input.lines() {
        let replaced = code_str.replace('\\', "\\\\").replace('\"', "\\\"");
        let mem_str = reg.replace_all(&replaced, "----");
        code += code_str.len();
        mem += mem_str.len() + 2;
    }

    mem - code
}

#[cfg(test)]
mod tests {
    use crate::{puzzle_one, puzzle_two};

    #[test]
    fn test_puzzle_one() {
        let actual = puzzle_one(
            r#"""
"abc"
"aaa\"aaa"
"\x27""#,
        );
        assert_eq!(actual, 12);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = puzzle_two(
            r#"""
"abc"
"aaa\"aaa"
"\x27""#,
        );
        assert_eq!(actual, 19);
    }
}
