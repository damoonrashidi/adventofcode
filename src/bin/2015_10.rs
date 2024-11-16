fn main() {
    let input = include_str!("../../inputs/2015/10.txt").trim();

    let mut first: String = input.to_string();
    for _ in 0..40 {
        first = puzzle_one(&first);
    }

    println!("puzzle one: {}", first.len());

    let mut second: String = input.to_string();
    for _ in 0..50 {
        second = puzzle_one(&second);
    }

    println!("puzzle two: {}", second.len());
}

fn puzzle_one(input: &str) -> String {
    let mut chars = input.chars();
    let mut current = chars.next().unwrap();
    let mut new: Vec<String> = vec![];
    let mut acc = 1;

    for next in chars {
        if next == current {
            acc += 1;
        } else {
            new.push(format!("{acc}"));
            new.push(current.to_string());
            current = next;
            acc = 1;
        }
    }
    new.push(format!("{acc}"));
    new.push(current.to_string());
    new.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::puzzle_one;

    #[test]
    fn test_puzzle_one() {
        assert_eq!(puzzle_one("1"), "11");
        assert_eq!(puzzle_one("11"), "21");
        assert_eq!(puzzle_one("21"), "1211");
        assert_eq!(puzzle_one("1211"), "111221");
        assert_eq!(puzzle_one("111221"), "312211");
    }
}
