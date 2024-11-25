use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/2016/3.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|w| w.parse::<usize>().unwrap())
                .sorted()
                .collect::<Vec<_>>()
        })
        .filter(|tri| tri[0] + tri[1] > tri[2])
        .count()
}

fn puzzle_two(input: &str) -> usize {
    let mut count = 0;
    let values = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    for x in 0..3 {
        for y in (0..values.len()).step_by(3) {
            let a = values[y][x];
            let b = values[y + 1][x];
            let c = values[y + 2][x];

            let mut sorted = [a, b, c];
            sorted.sort_unstable();

            if sorted[0] + sorted[1] > sorted[2] {
                count += 1;
            }
        }
    }

    count
}
