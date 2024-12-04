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

    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            let span_x = grid[y][x..(x + 4).min(width)].iter().collect::<String>();
            let span_y = grid[y..(y + 4).min(height)]
                .iter()
                .map(|r| r[x])
                .collect::<String>();
            let span_down = (0..4)
                .filter(|delta| {
                    (0..width).contains(&(x + delta)) && (0..height).contains(&(y + delta))
                })
                .map(|delta| grid[y + delta][x + delta])
                .collect::<String>();
            let span_up = (0..4)
                .filter(|delta| y.checked_sub(*delta).is_some() && x.checked_sub(*delta).is_some())
                .map(|delta| grid[y - delta][x - delta])
                .collect::<String>();
            if span_down == "XMAS" || span_down == "SAMX" {
                count += 1;
            }
            if span_up == "XMAS" || span_up == "SAMX" {
                count += 1;
            }
            if span_x == "XMAS" || span_x == "SAMX" {
                count += 1;
            }
            if span_y == "XMAS" || span_y == "SAMX" {
                count += 1;
            }
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
