fn main() {
    let input = include_str!("../../inputs/2025/1.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let mut cursor = 50isize;
    let mut count = 0;

    input
        .lines()
        .map(|x| {
            if x.starts_with('L') {
                -x.strip_prefix('L').unwrap().parse::<isize>().unwrap()
            } else {
                x.strip_prefix('R').unwrap().parse::<isize>().unwrap()
            }
        })
        .for_each(|value| {
            cursor += value;
            while cursor >= 100 {
                cursor -= 100;
            }
            while cursor < 0 {
                cursor += 100;
            }
            if cursor == 0 || cursor == 100 {
                cursor = 0;
                count += 1;
            }
        });

    count
}

fn puzzle_two(input: &str) -> usize {
    let mut cursor = 50isize;
    let mut count = 0;

    input
        .lines()
        .map(|x| {
            if x.starts_with('L') {
                -x.strip_prefix('L').unwrap().parse::<isize>().unwrap()
            } else {
                x.strip_prefix('R').unwrap().parse::<isize>().unwrap()
            }
        })
        .for_each(|value| {
            let laps = value.unsigned_abs() / 100;
            let remainder = value % 100;
            let mut target = remainder + cursor;

            if laps > 0 {
                count += laps;
            }

            if target == 100 {
                target = 0;
            }

            if target == 0 {
                count += 1
            } else if target < 0 {
                target = 100 + target;
                count += 1;
            } else if target >= 100 {
                target = target - 100;
                count += 1;
            }

            println!("{cursor} + {value} = {target}.  laps: {laps}");

            cursor = target;
        });

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        );
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_puzzle_r1000() {
        let actual = crate::puzzle_two(r"R1000");
        assert_eq!(actual, 10);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(
            r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        );
        assert_eq!(actual, 6);
    }
}
