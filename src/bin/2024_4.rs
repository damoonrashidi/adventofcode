fn main() {
    let input = include_str!("../../inputs/2024/4.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn is_opposite(a: char, b: char) -> bool {
    matches!((a, b), ('M', 'S') | ('S', 'M'))
}

fn puzzle_one(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();

    let get_span = |x: usize, y: usize, dx: isize, dy: isize| -> String {
        (0..4)
            .filter_map(|delta| {
                let ny = y.checked_add_signed(dy * delta)?;
                let nx = x.checked_add_signed(dx * delta)?;
                grid.get(ny)?.get(nx)
            })
            .collect()
    };

    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            count += [
                get_span(x, y, 1, 0),
                get_span(x, y, 0, 1),
                get_span(x, y, 1, 1),
                get_span(x, y, 1, -1),
            ]
            .iter()
            .filter(|&s| s == "XMAS" || s == "SAMX")
            .count();
        }
    }

    count
}

fn puzzle_two(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if (grid[y][x] == 'A')
                && is_opposite(grid[y - 1][x - 1], grid[y + 1][x + 1])
                && is_opposite(grid[y - 1][x + 1], grid[y + 1][x - 1])
            {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(actual, 18);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(
            r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(actual, 9);
    }
}
