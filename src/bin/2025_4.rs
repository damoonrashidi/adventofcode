fn main() {
    let input = include_str!("../../inputs/2025/4.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;

    let max_x = map[0].len();
    let max_y = map.len();

    println!("x: {max_x}, y: {max_y}");

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] != '@' {
                continue;
            }

            let mut n = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if let (Some(ty), Some(tx)) =
                        (y.checked_add_signed(dy), x.checked_add_signed(dx))
                    {
                        if tx < max_x && ty < max_y && map[ty][tx] == '@' && (0, 0) != (dy, dx) {
                            n += 1;
                        }
                    }
                }
            }

            if n < 4 {
                count += 1;
            }
        }
    }

    count
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        );
        assert_eq!(actual, 13);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
