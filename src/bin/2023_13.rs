fn main() {
    let input = include_str!("../../inputs/2023/13.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let chunks = parse(input);

    let mut v = vec![];
    let mut h = vec![];

    for chunk in chunks {
        if let Some(y) = split_point(&chunk) {
            h.push(y);
        } else if let Some(x) = split_point(&rotate(&chunk)) {
            v.push(x);
        }
    }

    v.iter().sum::<usize>() + h.iter().map(|x| x * 100).sum::<usize>()
}

fn split_point(chunk: &[Vec<char>]) -> Option<usize> {
    for y in 0..chunk.len() - 1 {
        let row = &chunk[y];
        let next_row = &chunk[y + 1];

        if row == next_row {
            let mut reflective = false;
            let mut gen = 1;

            if y == chunk.len() - 2 || y == 0 {
                return Some(y + 1);
            }

            loop {
                let first = y.saturating_sub(gen);
                let second = (y + gen + 1).min(chunk.len() - 1);

                let col = &chunk[first];
                let next_col = &chunk[second];

                if col != next_col {
                    break;
                }

                if first == 0 || second == chunk.len() - 1 {
                    reflective = true;
                    break;
                }

                gen += 1;
            }
            if reflective {
                return Some(y + 1);
            }
        }
    }
    None
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|chunk| chunk.lines().map(|l| l.chars().collect()).collect())
        .collect()
}

fn rotate<T: Clone + Copy>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut rotated = vec![vec![matrix[0][0]; rows]; cols];

    (0..rows).for_each(|i| {
        (0..cols).for_each(|j| {
            rotated[j][i] = matrix[i][j];
            rotated[j].reverse();
        });
    });

    rotated
}
