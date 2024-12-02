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

    for _ in 0..100 {
        let clone = map.clone();
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                let count = (-1..=1)
                    .flat_map(|dy| (-1..=1).map(move |dx| (dy, dx)))
                    .filter(|&(dy, dx)| !(dy == 0 && dx == 0))
                    .filter_map(|(dy, dx)| {
                        let ny = y.checked_add_signed(dy)?;
                        let nx = x.checked_add_signed(dx)?;
                        clone.get(ny)?.get(nx)
                    })
                    .filter(|cell| **cell)
                    .count();
                if clone[y][x] {
                    map[y][x] = count == 2 || count == 3;
                } else {
                    map[y][x] = count == 3;
                }
            }
        }
        if map == clone {
            break;
        }
    }

    map.iter().flatten().filter(|n| **n).count()
}

fn puzzle_two(input: &str) -> usize {
    let mut map = parse(input);
    let height = map.len() - 1;
    let width = map[0].len() - 1;

    for _ in 0..100 {
        map[0][0] = true;
        map[height][0] = true;
        map[height][width] = true;
        map[0][width] = true;
        let clone = map.clone();
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                let count = (-1..=1)
                    .flat_map(|dy| (-1..=1).map(move |dx| (dy, dx)))
                    .filter(|&(dy, dx)| !(dy == 0 && dx == 0))
                    .filter_map(|(dy, dx)| {
                        let ny = y.checked_add_signed(dy)?;
                        let nx = x.checked_add_signed(dx)?;
                        clone.get(ny)?.get(nx)
                    })
                    .filter(|cell| **cell)
                    .count();
                if clone[y][x] {
                    map[y][x] = count == 2 || count == 3;
                } else {
                    map[y][x] = count == 3;
                }
            }
        }

        if map == clone {
            break;
        }
    }

    map[0][0] = true;
    map[height][0] = true;
    map[height][width] = true;
    map[0][width] = true;

    map.iter().flatten().filter(|n| **n).count()
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
        let actual = crate::puzzle_two(
            r".#.#.#
...##.
#....#
..#...
#.#..#
####..",
        );
        assert_eq!(actual, 17);
    }
}
