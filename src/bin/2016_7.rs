fn main() {
    let input = include_str!("../../inputs/2016/7.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn supports_tls(line: &str) -> bool {
    let list = line.chars().collect::<Vec<char>>();
    let mut is_inside = false;
    let mut has_outside = false;
    for i in 0..list.len() - 3 {
        if list[i] == '[' {
            is_inside = true;
        } else if list[i] == ']' {
            is_inside = false;
        }
        if list[i] == list[i + 3]
            && list[i + 1] == list[i + 2]
            && list[i] != list[i + 2]
            && is_inside
        {
            return false;
        } else if list[i] == list[i + 3] && list[i + 1] == list[i + 2] && list[i] != list[i + 1] {
            has_outside = true;
        }
    }
    has_outside
}

fn supports_ssl(line: &str) -> bool {
    let list = line.chars().collect::<Vec<_>>();
    let mut is_inside = false;
    let mut outsides = vec![];
    let mut insides = vec![];

    for i in 0..list.len() - 2 {
        if list[i] == '[' {
            is_inside = true;
        } else if list[i] == ']' {
            is_inside = false;
        }
        if list[i] == list[i + 2] && !is_inside {
            outsides.push(vec![list[i], list[i + 1], list[i + 2]]);
        } else if list[i] == list[i + 2] {
            insides.push(vec![list[i], list[i + 1], list[i + 2]]);
        }
    }

    outsides.iter().any(|o| {
        insides
            .iter()
            .any(|i| [o[0], o[1], o[0]] == [i[1], i[0], i[1]])
    })
}

fn puzzle_one(input: &str) -> usize {
    input.lines().filter(|line| supports_tls(line)).count()
}

fn puzzle_two(input: &str) -> usize {
    input.lines().filter(|line| supports_ssl(line)).count()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_supports_tsl() {
        assert!(crate::supports_tls("ioxxoj[asdfgh]zxcvbn"));
        assert!(crate::supports_tls("abba[mnop]qrst"));
        assert!(!crate::supports_tls("aaaa[qwer]tyui"));
        assert!(!crate::supports_tls("abcd[bddb]xyyx"));
    }

    #[test]
    fn test_supports_ssl() {
        assert!(crate::supports_ssl("aba[bab]xyz"));
        assert!(crate::supports_ssl("aaa[kek]eke"));
        assert!(crate::supports_ssl("zazbz[bzb]cdb"));
        assert!(!crate::supports_ssl("xyx[xyx]xyx"));
    }

    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r"abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn",
        );
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
