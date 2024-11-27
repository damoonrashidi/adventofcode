use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/2016/6.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> String {
    let parsed = input
        .lines()
        .into_iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut password = vec![];

    for x in 0..parsed[0].len() {
        let mut map = HashMap::new();
        for y in 0..parsed.len() {
            let chr = parsed[y][x];
            if let Some(count) = map.get(&chr) {
                map.insert(chr, count + 1);
            } else {
                map.insert(chr, 1);
            }
        }
        let mut most = 0;
        let mut lead = ' ';
        map.iter_mut().for_each(|(chr, count)| {
            if *count > most {
                most = *count;
                lead = *chr;
            }
        });
        password.push(lead);
    }

    password.iter().collect()
}

fn puzzle_two(input: &str) -> String {
    let parsed = input
        .lines()
        .into_iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut password = vec![];

    for x in 0..parsed[0].len() {
        let mut map = HashMap::new();
        for y in 0..parsed.len() {
            let chr = parsed[y][x];
            if let Some(count) = map.get(&chr) {
                map.insert(chr, count + 1);
            } else {
                map.insert(chr, 1);
            }
        }
        let mut least = isize::MAX;
        let mut lead = ' ';
        map.iter_mut().for_each(|(chr, count)| {
            if *count < least {
                least = *count;
                lead = *chr;
            }
        });
        password.push(lead);
    }

    password.iter().collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar",
        );
        assert_eq!(actual, "easter".to_string());
        }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(
            "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar",
        );
        assert_eq!(actual, "advent".to_string());
    }
}
