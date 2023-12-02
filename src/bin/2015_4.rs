fn main() {
    let input = include_str!("../inputs/2015_4.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle one: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let mut i = 1;
    while !format!("{:x}", md5::compute(format!("{input}{i}"))).starts_with("00000") {
        i += 1;
    }
    i
}
fn puzzle_two(input: &str) -> usize {
    let mut i = 1;
    while !format!("{:x}", md5::compute(format!("{input}{i}"))).starts_with("000000") {
        i += 1;
    }
    i
}
