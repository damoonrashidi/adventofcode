fn main() {
    let input = include_str!("../inputs/2015/2.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let dims = line
                .split('x')
                .map(|dim| dim.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let (l, w, h) = (dims[0], dims[1], dims[2]);
            let (first, second, third) = ((l * w), (w * h), (h * l));
            let slack = first.min(second).min(third);

            first * 2 + second * 2 + third * 2 + slack
        })
        .sum()
}

fn puzzle_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let dims = line
                .split('x')
                .map(|dim| dim.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let (l, w, h) = (dims[0], dims[1], dims[2]);
            let bow = l * w * h;

            let mut sorted = [l, w, h];
            sorted.sort_unstable();

            bow + sorted[0] * 2 + sorted[1] * 2
        })
        .sum()
}
