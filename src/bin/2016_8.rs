fn main() {
    let input = include_str!("../../inputs/2016/8.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    puzzle_two(input);
}

#[derive(Debug)]
enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let words = value.split_whitespace().collect::<Vec<_>>();

        if words[0] == "rect" {
            let dims = words[1].split_once('x').unwrap();
            let x = dims.0.parse::<usize>().unwrap();
            let y = dims.1.parse::<usize>().unwrap();
            return Instruction::Rect(x, y);
        }

        let start = words[2]
            .split_once('=')
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        let amount = words[4].parse::<usize>().unwrap();

        if words[1] == "column" {
            Instruction::RotateCol(start, amount)
        } else {
            Instruction::RotateRow(start, amount)
        }
    }
}

fn puzzle_one(input: &str) -> usize {
    let mut screen = [[false; WIDTH]; HEIGHT];

    for instruction in input.lines().map(Instruction::from) {
        match instruction {
            Instruction::Rect(to_x, to_y) => {
                (0..to_x).for_each(|x| {
                    (0..to_y).for_each(|y| {
                        screen[y][x] = true;
                    });
                });
            }
            Instruction::RotateRow(row, amount) => {
                let tmp = screen;

                (0..WIDTH).for_each(|i| {
                    let end = (i + amount) % WIDTH;
                    screen[row][end] = tmp[row][i];
                });
            }
            Instruction::RotateCol(col, amount) => {
                let tmp = screen;

                (0..HEIGHT).for_each(|i| {
                    let end = (i + amount) % HEIGHT;
                    screen[end][col] = tmp[i][col];
                });
            }
        }
    }

    screen.iter().flatten().filter(|c| **c).count()
}

fn puzzle_two(input: &str) {
    let mut screen = [[false; WIDTH]; HEIGHT];

    for instruction in input.lines().map(Instruction::from) {
        match instruction {
            Instruction::Rect(to_x, to_y) => {
                (0..to_x).for_each(|x| {
                    (0..to_y).for_each(|y| {
                        screen[y][x] = true;
                    });
                });
            }
            Instruction::RotateRow(row, amount) => {
                let tmp = screen;

                (0..WIDTH).for_each(|i| {
                    let end = (i + amount) % WIDTH;
                    screen[row][end] = tmp[row][i];
                });
            }
            Instruction::RotateCol(col, amount) => {
                let tmp = screen;

                (0..HEIGHT).for_each(|i| {
                    let end = (i + amount) % HEIGHT;
                    screen[end][col] = tmp[i][col];
                });
            }
        }
    }

    for row in screen {
        for column in row {
            print!("{}", if column { 'x' } else { ' ' });
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r"rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1",
        );
        assert_eq!(actual, 6);
    }
}
