fn main() {
    let input = include_str!("../../inputs/2016/2.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let mut answer = vec![];
    let keypad = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    let mut x: usize = 1;
    let mut y: usize = 1;

    for line in input.lines() {
        for c in line.chars() {
            match c {
                'U' => {
                    y = y.saturating_sub(1);
                }
                'R' => {
                    x = (x + 1).min(2);
                }
                'L' => {
                    x = x.saturating_sub(1);
                }
                'D' => {
                    y = (y + 1).min(2);
                }
                _ => unreachable!(),
            };
        }

        answer.push(keypad[y][x]);
    }

    answer
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn puzzle_two(input: &str) -> String {
    let mut answer = vec![];
    let keypad = [
        ['0', '0', '1', '0', '0'],
        ['0', '2', '3', '4', '0'],
        ['5', '6', '7', '8', '9'],
        ['0', 'A', 'B', 'C', '0'],
        ['0', '0', 'D', '0', '0'],
    ];

    let mut x: usize = 0;
    let mut y: usize = 2;

    for line in input.lines() {
        for c in line.chars() {
            let old_x = x;
            let old_y = y;

            match c {
                'U' => {
                    y = y.saturating_sub(1);
                }
                'R' => {
                    x = (x + 1).min(4);
                }
                'L' => {
                    x = x.saturating_sub(1);
                }
                'D' => {
                    y = (y + 1).min(4);
                }
                _ => unreachable!(),
            };

            if keypad[y][x] == '0' {
                x = old_x;
                y = old_y;
            }
        }

        answer.push(keypad[y][x]);
    }

    answer
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::{puzzle_one, puzzle_two};

    #[test]
    fn test_puzzle_one() {
        let actual = puzzle_one(
            r"ULL
RRDDD
LURDL
UUUUD",
        );
        assert_eq!(actual, 1985);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = puzzle_two(
            r"ULL
RRDDD
LURDL
UUUUD",
        );
        assert_eq!(actual, "5DB3".to_string());
    }
}
