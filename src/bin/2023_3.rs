use std::collections::HashMap;

fn main() {
    let input = include_str!("../inputs/2023_3.txt").trim();

    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    println!("puzzle one: {}", puzzle_one(&map));
    println!("puzzle two: {}", puzzle_two(&map));
}

fn puzzle_one(map: &Vec<Vec<char>>) -> usize {
    let mut nums: Vec<usize> = vec![];

    let mut y = 0;
    let mut x = 0;
    while (0..map.len()).contains(&y) {
        while (0..map[0].len()).contains(&x) {
            let mut num = vec![];
            while x < map[0].len() && map[y][x].is_ascii_digit() {
                num.push(map[y][x]);
                x += 1;
            }
            if !num.is_empty() {
                let num_str = num.into_iter().collect::<String>();
                let num: usize = num_str.parse().unwrap();
                if is_adjacent_to_special(x - num_str.len(), x, y, map) {
                    nums.push(num);
                }
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }

    nums.into_iter().sum()
}

fn puzzle_two(map: &Vec<Vec<char>>) -> usize {
    let mut gear_map: HashMap<String, Vec<usize>> = HashMap::new();

    let mut y = 0;
    let mut x = 0;
    while (0..map.len()).contains(&y) {
        while (0..map[0].len()).contains(&x) {
            let mut num = vec![];
            while x < map[0].len() && map[y][x].is_ascii_digit() {
                num.push(map[y][x]);
                x += 1;
            }
            if !num.is_empty() {
                let num_str = num.into_iter().collect::<String>();
                let num: usize = num_str.parse().unwrap();

                let gears = get_adjacent_gear_locations(x - num_str.len(), x, y, map);
                for gear in gears {
                    if let Some(list) = gear_map.get_mut(&gear) {
                        list.push(num);
                    } else {
                        gear_map.insert(gear, vec![num]);
                    }
                }
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }

    gear_map
        .values()
        .filter(|list| list.len() == 2)
        .map(|list| list.iter().product::<usize>())
        .sum()
}

fn get_adjacent_gear_locations(
    x_min: usize,
    x_max: usize,
    y: usize,
    map: &[Vec<char>],
) -> Vec<String> {
    let mut locations = vec![];

    let y_start = y.saturating_sub(1);
    let y_end = (y + 1).min(map.len() - 1);
    let x_start = x_min.saturating_sub(1);
    let x_end = x_max.min(map[0].len() - 1);

    (y_start..=y_end).for_each(|y| {
        (x_start..=x_end).for_each(|x| {
            if map[y][x] == '*' {
                locations.push(format!("{x}:{y}"));
            }
        });
    });

    locations
}

fn is_adjacent_to_special(x_min: usize, x_max: usize, y: usize, map: &[Vec<char>]) -> bool {
    let y_start = y.saturating_sub(1);
    let y_end = (y + 1).min(map.len() - 1);
    let x_start = x_min.saturating_sub(1);
    let x_end = x_max.min(map[0].len() - 1);

    #[allow(clippy::needless_range_loop)]
    for y in y_start..=y_end {
        for x in x_start..=x_end {
            let c = map[y][x];
            if !(c.is_ascii_digit() || c == '.') {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::puzzle_two;

    #[test]
    fn test() {
        let map = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let actual = puzzle_two(&map);
        assert_eq!(actual, 47835);
    }
}
