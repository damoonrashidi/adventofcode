fn main() {
    let input = include_str!("../../inputs/2023/14.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

#[derive(Debug, PartialEq)]
struct Coord {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

fn puzzle_one(input: &str) -> usize {
    let (mut balls, map) = parse(input);

    for ball in &mut balls {
        let mut y = ball.y;
        let mut seen_balls = 0;
        while y > 0 {
            y -= 1;
            if map[y][ball.x] == 'O' {
                seen_balls += 1;
            } else if map[y][ball.x] == '#' {
                y += 1;
                break;
            }
        }
        ball.y = y + seen_balls;
    }

    balls.iter().map(|b| map.len() - b.y).sum()
}

#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn puzzle_two(input: &str) -> usize {
    let (mut balls, map) = parse(input);

    for i in 0..1 {
        let instructions = [(0, -1), (-1, 0), (0, 1), (1, 0)];

        let y_max = map.len() - 1;
        let x_max = map[0].len() - 1;

        if i % 1_000_000 == 0 {
            println!("{}%", i / 1_000_000);
        }

        for (d_x, d_y) in instructions {
            println!("going {d_x},{d_y}");
            for ball in &mut balls {
                let mut y = ball.y as isize;
                let mut x = ball.x as isize;
                let mut seen_balls = 0;

                while y > 0 && y < y_max as isize && x > 0 && x < x_max as isize {
                    y += d_y;
                    x += d_x;

                    if map[y as usize][x as usize] == 'O' {
                        seen_balls += 1;
                    } else if map[y as usize][x as usize] == '#' {
                        y -= d_y;
                        x -= d_x;
                        break;
                    }
                }
                ball.y = (y + seen_balls) as usize;
                ball.x = (x + seen_balls) as usize;
            }

            print_updated(map.clone(), &balls);
        }
    }

    println!(
        "{:?}",
        balls
            .iter()
            .map(|b| map.len().saturating_sub(b.y))
            .collect::<Vec<usize>>()
    );

    balls.iter().map(|b| map.len().saturating_sub(b.y)).sum()
}

fn print_updated(mut map: Vec<Vec<char>>, balls: &[Coord]) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                map[y][x] = '.';
            }
            if balls.contains(&Coord { x, y }) {
                map[y][x] = 'O';
            }
        }
    }
    for row in map {
        println!("{row:?}");
    }
}

fn parse(input: &str) -> (Vec<Coord>, Vec<Vec<char>>) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut coords = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                coords.push(Coord { x, y });
            }
        }
    }

    (coords, map)
}

#[cfg(test)]
mod tests {
    use crate::puzzle_two;

    #[test]
    fn test_puzzle_one() {
        let actual = puzzle_two(
            r".....#....
....#....#
.#...##...
...#......
....O...#.
..#....#.#
.....#....
..........
#....###..
#....#....",
        );
        assert_eq!(actual, 64);
    }
}
