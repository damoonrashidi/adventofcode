fn main() {
    let input = include_str!("../../inputs/2016/5.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> String {
    let mut password: [char; 8] = [' '; 8];
    let mut c = 0;

    for i in 0..usize::MAX {
        let computed = format!("{:x}", md5::compute(format!("{input}{i}")));
        if computed.starts_with("00000") {
            let chr = computed.get(5..6).unwrap().chars().next().unwrap();
            password[c] = chr;
            c += 1;
            if c == 8 {
                break;
            }
        }
    }

    password.into_iter().collect::<String>()
}

fn puzzle_two(input: &str) -> String {
    let mut password: [char; 8] = [' '; 8];

    for i in 0..usize::MAX {
        let computed = format!("{:x}", md5::compute(format!("{input}{i}")));
        if computed.starts_with("00000") {
            if let Some(c) = computed
                .get(5..6)
                .unwrap()
                .chars()
                .next()
                .unwrap()
                .to_digit(10)
            {
                if c < 8 && password[c as usize] == ' ' {
                    let chr = computed.get(6..7).unwrap().chars().next().unwrap();
                    password[c as usize] = chr;
                    println!("{password:?}");
                    if !password.contains(&' ') {
                        break;
                    }
                }
            }
        }
    }

    password.into_iter().collect::<String>()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(r"abc");
        assert_eq!(actual, "18f47a30".to_string());
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(r"");
        assert_eq!(actual, "0".to_string());
    }
}
