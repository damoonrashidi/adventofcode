fn main() {
    let input = include_str!("../../inputs/2019/1.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn fuel_cost(mass: usize) -> usize {
    if (mass / 3).saturating_sub(2) == 0 {
        return mass;
    }

    mass + fuel_cost(mass / 3 - 2)
}

fn puzzle_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap() / 3 - 2)
        .sum::<usize>()
}

fn puzzle_two(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mass = line.parse::<usize>().unwrap();
            fuel_cost(mass) - mass
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(r"100756");
        assert_eq!(actual, 33583);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = crate::puzzle_two(r"1969");
        assert_eq!(actual, 966);
    }
}
