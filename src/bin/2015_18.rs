fn main() {
    let input = include_str!("../../inputs/2015/18.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn puzzle_one(input: &str) -> usize {
    let mut map = parse(input);

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            print!("{}", if map[y][x] { "#" } else { "." });
        }
        println!();
    }
    println!();

    for _ in 0..2 {
        let clone = map.clone();
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                let mut count: usize = 0;
                for dy in y.saturating_sub(1)..=(y + 1).min(map.len() - 1) {
                    for dx in x.saturating_sub(1)..=(x + 1).min(map[0].len() - 1) {
                        print!(
                            "y: {:?} x: {:?} -- ",
                            y.saturating_sub(1)..=(y + 1).min(map.len() - 1),
                            x.saturating_sub(1)..=(x + 1).min(map[0].len() - 1)
                        );
                        if clone[dy][dx] && dy != y && dx != x {
                            count += 1;
                        }
                    }
                    println!("count: {count}");
                }
                if clone[y][x] {
                    map[y][x] = count == 2 || count == 3;
                } else {
                    map[y][x] = count == 3;
                }
            }
        }

        for y in 0..map.len() {
            for x in 0..map[0].len() {
                print!("{}", if map[y][x] { "#" } else { "." });
            }
            println!();
        }
        println!();
    }

    map.iter().flatten().filter(|n| **n).count()
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r".#.#.#
...##.
#....#
..#...
#.#..#
####..",
        );
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
