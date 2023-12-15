use std::str::Chars;

enum Instruction<'a> {
    Assign(&'a str, usize),
    Remove(&'a str),
}

fn main() {
    let input = include_str!("../../inputs/2023/15.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> u32 {
    input.split(',').map(|word| hash(word.chars())).sum()
}

fn puzzle_two(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    input.split(',').for_each(|word| {
        let instruction = make_instruction(word);
        match instruction {
            Instruction::Assign(label, value) => {
                let i = hash(label.to_string().chars()) as usize;
                if let Some(f_i) = boxes[i].iter().position(|(l, _)| l == &label) {
                    boxes[i][f_i] = (label, value);
                } else {
                    boxes[i].push((label, value));
                }
            }
            Instruction::Remove(label) => {
                let i = hash(label.to_string().chars()) as usize;
                if let Some(f_i) = boxes[i].iter().position(|(l, _)| l == &label) {
                    boxes[i].remove(f_i);
                }
            }
        };
    });

    boxes
        .into_iter()
        .enumerate()
        .map(|(bx, b)| {
            b.into_iter()
                .enumerate()
                .map(|(slot, (_, v))| (bx + 1) * (slot + 1) * v)
                .sum::<usize>()
        })
        .sum()
}

fn make_instruction(word: &str) -> Instruction {
    if word.contains('=') {
        let (a, b) = word.split_once('=').unwrap();
        return Instruction::Assign(a, b.parse().unwrap());
    }
    return Instruction::Remove(word.strip_suffix('-').unwrap());
}

fn hash(word: Chars) -> u32 {
    word.fold(0, |v, c| (v + c as u32) * 17 % 256)
}

#[cfg(test)]
mod tests {
    use crate::{puzzle_one, puzzle_two};

    #[test]
    fn test_puzzle_one() {
        let actual = puzzle_one(r"HASH");
        assert_eq!(actual, 52);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = puzzle_two(r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(actual, 145);
    }
}
