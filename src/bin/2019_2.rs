fn main() {
    let input = include_str!("../../inputs/2019/2.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let mut codes = input
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    codes[1] = 12;
    codes[2] = 2;

    for i in (0..codes.len()).step_by(4) {
        match codes.get(i) {
            Some(1) => {
                let output = codes[i + 3];
                let a = codes[i + 1];
                let b = codes[i + 2];
                codes[output] = codes[a] + codes[b];
            }
            Some(2) => {
                let output = codes[i + 3];
                let a = codes[i + 1];
                let b = codes[i + 2];
                codes[output] = codes[a] * codes[b];
            }
            Some(99) => {
                break;
            }
            _ => unreachable!(),
        }
    }

    codes[0]
}

fn puzzle_two(input: &str) -> usize {
    let codes = input
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for x in 0..100 {
        for y in 0..100 {
            let mut codes = codes.clone();
            codes[1] = x;
            codes[2] = y;
            for i in (0..codes.len()).step_by(4) {
                match codes.get(i) {
                    Some(1) => {
                        let output = codes[i + 3];
                        let a = codes[i + 1];
                        let b = codes[i + 2];
                        codes[output] = codes[a] + codes[b];
                    }
                    Some(2) => {
                        let output = codes[i + 3];
                        let a = codes[i + 1];
                        let b = codes[i + 2];
                        codes[output] = codes[a] * codes[b];
                    }
                    Some(99) => {
                        break;
                    }
                    _ => unreachable!(),
                }
            }
            if codes[0] == 19_690_720 {
                return 100 * x + y;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        assert_eq!(crate::puzzle_one(r"1,9,10,3,2,3,11,0,99,30,40,50"), 3500);
        assert_eq!(crate::puzzle_one(r"1,0,0,0,99"), 2);
        assert_eq!(crate::puzzle_one(r"2,3,0,3,99"), 2);
        assert_eq!(crate::puzzle_one(r"2,4,4,5,99,0"), 2);
        assert_eq!(crate::puzzle_one(r"1,1,1,4,99,5,6,0,99"), 30);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
