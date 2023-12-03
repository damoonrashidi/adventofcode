use std::ops::Range;

fn main() {
    let input = include_str!("../inputs/2023_3.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle one: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let map = parse_map(input);
    let mut nums: Vec<usize> = vec![];

    let mut y = 0;
    let mut x = 0;
    while (0..map.len()).contains(&y) {
        println!("Line: {}", y + 1);
        while (0..map[0].len()).contains(&x) {
            let mut num = vec![];
            while x < map[0].len() && map[y][x].is_ascii_digit() {
                num.push(map[y][x]);
                x += 1;
            }
            if !num.is_empty() {
                let num_str = num.into_iter().collect::<String>();
                let num: usize = num_str.parse().unwrap();
                #[allow(clippy::range_plus_one)]
                if is_adjacent_to_special(x - num_str.len()..x + 1, y, &map) {
                    println!("{num} IS adjacent");
                    nums.push(num);
                } else {
                    println!("{num} isn't adjacent to any special chars");
                }
            }
            x += 1;
        }
        x = 0;
        y += 1;
        println!("\n\n");
    }

    nums.into_iter().sum()
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

#[allow(unused)]
fn is_adjacent_to_special(x: Range<usize>, y: usize, map: &[Vec<char>]) -> bool {
    let start = x.start.max(1);
    let end = x.end.min(map[0].len() - 1);

    if y > 0 {
        let above = map.get(y - 1).unwrap();
        if above[(start - 1)..end].iter().any(|c| is_special_char(*c)) {
            return true;
        }
    };
    if y < map.len() - 1 {
        let below = map.get(y + 1).unwrap();
        if below[(start - 1)..end].iter().any(|c| is_special_char(*c)) {
            return true;
        }
    };
    if x.start > 0 {
        let left = map[y][x.start - 1];
        if is_special_char(left) {
            return true;
        }
    }
    if x.end < map[0].len() - 1 {
        let right = map[y][x.end - 1];
        if is_special_char(right) {
            return true;
        }
    }

    false
}

fn parse_map(map: &str) -> Vec<Vec<char>> {
    map.lines().map(|line| line.chars().collect()).collect()
}

fn is_special_char(c: char) -> bool {
    !(c.is_ascii_digit() || c == '.')
}

#[cfg(test)]
mod tests {
    use crate::puzzle_one;

    #[test]
    fn test() {
        let actual = puzzle_one(
            r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
                .trim(),
        );

        assert_eq!(actual, 4361);
    }

    #[test]
    fn two() {
        let actual = puzzle_one(
            r"
.......206......*....668..728$............529..........*93........................-...833.713..
..........*585...969............810..522............866..............254.....570........*....*..."
                .trim(),
        );

        assert_eq!(actual, 206 + 728 + 93 + 833 + 713 + 585 + 969 + 866);
    }
}
