use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/2024/1.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let mut a = vec![];
    let mut b = vec![];
    let mut ans = 0;

    input.lines().for_each(|line| {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let f = split[0].parse::<usize>().unwrap();
        let s = split[1].parse::<usize>().unwrap();
        a.push(f);
        b.push(s);
    });

    a.sort_unstable();
    b.sort_unstable();

    for i in 0..a.len() {
        ans += a[i].abs_diff(b[i]);
    }

    ans
}

fn puzzle_two(input: &str) -> usize {
    let mut list: Vec<usize> = vec![];
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut ans = 0;

    input.lines().for_each(|line| {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let f = split[0].parse::<usize>().unwrap();
        let s = split[1].parse::<usize>().unwrap();
        list.push(f);

        *map.entry(s).or_insert(0) += 1;
    });

    (0..list.len()).for_each(|i| {
        let count = map.get(&list[i]).unwrap_or(&0);
        ans += list[i] * count;
    });

    ans
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r"3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(actual, 11);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(
            r"3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(actual, 31);
    }
}
