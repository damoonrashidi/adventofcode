fn main() {
    let input = include_str!("../inputs/2022/1.txt");

    println!("{}", puzzle_one(input));
}

fn puzzle_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|inventory| {
            inventory
                .lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}
