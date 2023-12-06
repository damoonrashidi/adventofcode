use std::collections::HashMap;

fn main() {
    let races = [(58, 478), (99, 2232), (64, 1019), (69, 1071)];
    println!("puzzle one: {}", puzzle_one(&races));

    let time = 58_996_469;
    let dist = 478_223_210_191_071;
    println!("puzzle two: {}", puzzle_two(time, dist));
}

fn puzzle_one(races: &[(usize, usize)]) -> usize {
    let mut wins = HashMap::new();
    for (i, (time, dist)) in races.iter().enumerate() {
        for charge in 0..time - 1 {
            let remaining = time - charge;
            if charge * remaining > *dist {
                if let Some(for_race) = wins.get(&i) {
                    wins.insert(i, for_race + 1);
                } else {
                    wins.insert(i, 1);
                }
            }
        }
    }

    wins.values().product()
}

fn puzzle_two(time: usize, dist: usize) -> usize {
    let mut wins = 0;
    for charge in 0..time - 1 {
        let remaining = time - charge;
        if charge * remaining > dist {
            wins += 1;
        }
    }
    wins
}

#[cfg(test)]
mod tests {
    use crate::puzzle_two;

    #[test]
    fn test() {
        let actual = puzzle_two(71530, 940_200);
        assert_eq!(actual, 71503);
    }
}
