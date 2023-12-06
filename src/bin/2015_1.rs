fn main() {
    let input = include_str!("../inputs/2015/1.txt").trim();
    let first = puzzle_one(input);
    let second = puzzle_two(input);

    println!("first: {first}");
    println!("second: {second}");
}

fn puzzle_one(input: &str) -> i64 {
    input.chars().map(|c| if c == '(' { 1 } else { -1 }).sum()
}

fn puzzle_two(input: &str) -> usize {
    let mut i = 0;
    let mut ans = 0;
    for c in input.chars() {
        ans += if c == '(' { 1 } else { -1 };
        i += 1;
        if ans < 0 {
            return i;
        }
    }
    0
}
