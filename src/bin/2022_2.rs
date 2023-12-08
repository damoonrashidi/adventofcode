fn main() {
    let input = include_str!("../../inputs/2022/2.txt");
    println!("{}", puzzle_one(input));
    println!("{}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> u32 {
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

fn puzzle_two(input: &str) -> u32 {
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
        ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0, //scissor -> paper
        ('B', 'Y') | ('A', 'X') | ('C', 'Z') => 3, //scissor -> scissor
        ('B', 'Z') | ('C', 'X') | ('A', 'Y') => 6, //rock -> paper
        _ => unreachable!(),
    }
}

fn should_pick(op_move: char, me_move: char) -> u32 {
    match (op_move, me_move) {
        ('A', 'X') => move_score('Z'),     //rock -> lose
        ('A', 'Y') => 3 + move_score('X'), //rock -> draw
        ('A', 'Z') => 6 + move_score('Y'), //rock -> win
        ('B', 'X') => move_score('X'),     //paper -> lose
        ('B', 'Y') => 3 + move_score('Y'), //paper -> draw
        ('B', 'Z') => 6 + move_score('Z'), //paper -> win
        ('C', 'X') => move_score('Y'),     //scissor -> lose
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
        let actual = puzzle_one("A Y\nB X\nC Z");
        assert_eq!(actual, 15);
    }
}
