fn main() {
    let input = include_str!("../../inputs/2016/9.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let mut result = vec![];
    let mut i = 0;
    let chars = input.chars().collect::<Vec<_>>();

    while i < chars.len() {
        if chars[i] == '(' {
            let mut inside = vec![];
            while chars[i + 1] != ')' {
                inside.push(chars[i + 1]);
                i += 1;
            }
            let split = inside.iter().position(|c| c == &'x').unwrap();
            let reach = inside[0..split]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            let repeat = inside[split + 1..]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            let to_add = &chars[i + 2..i + 2 + reach];

            (0..repeat).for_each(|_| result.extend_from_slice(to_add));

            i += 2 + reach;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result.len()
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        assert_eq!(crate::puzzle_one("ADVENT"), "ADVENT".to_string().len());
        assert_eq!(crate::puzzle_one("A(1x5)BC"), "ABBBBBC".to_string().len());
        assert_eq!(crate::puzzle_one("(3x3)XYZ"), "XYZXYZXYZ".to_string().len());
        assert_eq!(crate::puzzle_one("(6x1)(1x3)A"), "(1x3)A".to_string().len());
        assert_eq!(
            crate::puzzle_one("X(8x2)(3x3)ABCY"),
            "X(3x3)ABC(3x3)ABCY".to_string().len()
        );
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
