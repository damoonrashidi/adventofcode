use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).unwrap();

    println!("{}", puzzle_one(file.clone()));
    println!("{}", puzzle_two(file));
}

fn puzzle_one(input: String) -> u32 {
    let moves = input
        .lines()
        .map(|line| {
            let split = line.chars().collect::<Vec<char>>();
            (split[0], split[2])
        })
        .collect::<Vec<(char, char)>>();

    moves
        .into_iter()
        .map(|(op_move, me_move)| -> u32 { outcome(op_move, me_move) + move_score(me_move) })
        .sum()
}

fn puzzle_two(input: String) -> u32 {
    let moves = input
        .lines()
        .map(|line| {
            let split = line.chars().collect::<Vec<char>>();
            (split[0], split[2])
        })
        .collect::<Vec<(char, char)>>();

    moves
        .into_iter()
        .map(|(op_move, me_move)| -> u32 { should_pick(op_move, me_move) })
        .sum()
}

fn outcome(op_move: char, me_move: char) -> u32 {
    match (op_move, me_move) {
        ('A', 'X') => 3, //rock -> rock
        ('A', 'Y') => 6, //rock -> paper
        ('A', 'Z') => 0, //rock -> scissor
        ('B', 'X') => 0, //paper -> rock
        ('B', 'Y') => 3, //paper -> paper
        ('B', 'Z') => 6, //paper -> scissor
        ('C', 'X') => 6, //scissor -> rock
        ('C', 'Y') => 0, //scissor -> paper
        ('C', 'Z') => 3, //scissor -> scissor
        _ => unreachable!(),
    }
}

fn should_pick(op_move: char, me_move: char) -> u32 {
    match (op_move, me_move) {
        ('A', 'X') => 0 + move_score('Z'), //rock -> lose
        ('A', 'Y') => 3 + move_score('X'), //rock -> draw
        ('A', 'Z') => 6 + move_score('Y'), //rock -> win
        ('B', 'X') => 0 + move_score('X'), //paper -> lose
        ('B', 'Y') => 3 + move_score('Y'), //paper -> draw
        ('B', 'Z') => 6 + move_score('Z'), //paper -> win
        ('C', 'X') => 0 + move_score('Y'), //scissor -> lose
        ('C', 'Y') => 3 + move_score('Z'), //scissor -> draw
        ('C', 'Z') => 6 + move_score('X'), //scissor -> win
        _ => unreachable!(),
    }
}

fn move_score(me_move: char) -> u32 {
    match me_move {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle_one;

    #[test]
    fn day2_test1() {
        let actual = puzzle_one("A Y\nB X\nC Z".into());
        assert_eq!(actual, 15)
    }
}
