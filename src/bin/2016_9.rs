fn main() {
    let input = include_str!("../../inputs/2016/9.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

#[allow(unused)]
fn decompress(line: &str, range: Option<(usize, usize)>, val: String) -> String {
    val
}

fn puzzle_one(input: &str) -> usize {
    decompress(input, None, String::new()).len()
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        assert_eq!(
            crate::decompress("ADVENT", None, String::new()),
            "ADVENT".to_string()
        );
        assert_eq!(
            crate::decompress("A(1x5)BC", None, String::new()),
            "ABBBBBC".to_string()
        );
        assert_eq!(
            crate::decompress("(3x3)XYZ", None, String::new()),
            "XYZXYZXYZ".to_string()
        );
        assert_eq!(
            crate::decompress("(6x1)(1x3)A", None, String::new()),
            "(1x3)A".to_string()
        );
        assert_eq!(
            crate::decompress("X(8x2)(3x3)ABCY", None, String::new()),
            "X(3x3)ABC(3x3)ABCY".to_string()
        );
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
