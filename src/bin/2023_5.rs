use std::ops::Range;

fn main() {
    let input = include_str!("../inputs/2023/5.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let (first_line, _) = input.split_once('\n').unwrap();
    let seeds: Vec<usize> = first_line
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect();

    let map_tunnel = parse_maps(input);

    let mut outputs: Vec<usize> = vec![];

    for seed in seeds {
        let mut ins = seed;
        for ranges in &map_tunnel {
            ins = if let Some((to, from)) = ranges.iter().find(|(_, from)| from.contains(&ins)) {
                to.start + from.start.abs_diff(ins)
            } else {
                ins
            };
        }

        outputs.push(ins);
    }

    outputs
        .into_iter()
        .min()
        .expect("could not find any location")
}

fn puzzle_two(input: &str) -> usize {
    let (first_line, _) = input.split_once('\n').unwrap();
    let seeds_strs: Vec<&str> = first_line
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .collect();

    let mut seeds: Vec<Range<usize>> = vec![];

    for i in (0..seeds_strs.len() - 1).step_by(2) {
        let start = seeds_strs[i].parse().unwrap();
        let amount: usize = seeds_strs[i + 1].parse().unwrap();
        seeds.push(start..(start + amount));
    }

    let map_tunnel = parse_maps(input);
    let mut min = usize::MAX;

    for seed_range in seeds {
        for seed in seed_range {
            let mut ins = seed;
            for ranges in &map_tunnel {
                ins = if let Some((to, from)) = ranges
                    .iter()
                    .find(|(_, from)| (from.start..=from.end).contains(&ins))
                {
                    to.start + from.start.abs_diff(ins)
                } else {
                    ins
                };
            }

            if ins < min {
                min = ins;
            }
        }
    }

    min
}

fn parse_maps(input: &str) -> Vec<Vec<(Range<usize>, Range<usize>)>> {
    input
        .split("\n\n")
        .skip(1)
        .map(|chunk| {
            chunk
                .lines()
                .skip(1)
                .map(|line| {
                    let a = line
                        .split_whitespace()
                        .map(|str| str.parse().unwrap())
                        .collect::<Vec<usize>>();

                    (a[0]..a[0] + a[2], a[1]..a[1] + a[2])
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::puzzle_two;

    #[test]
    fn test() {
        let actual = puzzle_two(
            r"seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4",
        );
        assert_eq!(actual, 46);
    }
}
