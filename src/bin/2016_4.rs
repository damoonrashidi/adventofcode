use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/2016/4.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

struct Room {
    pub letters: HashMap<char, usize>,
    pub sector: usize,
    pub checksum: Vec<char>,
    pub raw: Vec<char>,
}

impl From<&str> for Room {
    fn from(value: &str) -> Self {
        let mut letters: HashMap<char, usize> = HashMap::new();
        let mut raw = vec![];
        let mut sector: Vec<char> = vec![];
        value.chars().take_while(|c| c != &'[').for_each(|c| {
            if c.is_alphabetic() {
                raw.push(c);
                *letters.entry(c).or_insert(1) += 1;
            } else if c.is_numeric() {
                sector.push(c);
            }
        });

        let checksum = value
            .chars()
            .skip_while(|c| c != &'[')
            .filter(|c| c.is_alphabetic())
            .collect::<Vec<char>>();

        Room {
            letters,
            sector: sector.iter().collect::<String>().parse::<usize>().unwrap(),
            checksum,
            raw,
        }
    }
}

impl Room {
    pub(self) fn is_real(&self) -> bool {
        let sorted = self
            .letters
            .iter()
            .sorted_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)))
            .collect::<Vec<_>>();

        for (i, char) in self.checksum.iter().enumerate() {
            if sorted[i].0 != char {
                return false;
            }
        }

        true
    }
}

fn puzzle_one(input: &str) -> usize {
    input
        .lines()
        .map(Room::from)
        .filter(Room::is_real)
        .map(|room| room.sector)
        .sum()
}

fn puzzle_two(input: &str) -> usize {
    for room in input.lines().map(Room::from) {
        #[allow(clippy::cast_possible_truncation)]
        let shifted = room
            .raw
            .iter()
            .map(|&c| {
                if c == '-' {
                    return ' ';
                }
                let offset = (room.sector % 26) as u32;
                let c_num = c as u32 - 'a' as u32;
                let shifted_num = (c_num + offset) % 26;
                char::from_u32(shifted_num + 'a' as u32).unwrap()
            })
            .collect::<String>();

        if shifted == "northpoleobjectstorage" {
            return room.sector;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::{puzzle_two, Room};

    #[test]
    fn test_is_valid() {
        let room = Room::from("aaaaa-bbb-z-y-x-123[abxyz]");
        assert!(room.is_real());

        let room = Room::from("a-b-c-d-e-f-g-h-987[abcde]");
        assert!(room.is_real());

        let room = Room::from("not-a-real-room-404[oarel]");
        assert!(room.is_real());

        let room = Room::from("totally-real-room-200[decoy]");
        assert!(!room.is_real());
    }

    #[test]
    fn test_puzzle_one() {
        let actual = crate::puzzle_one(
            r"aaaaa-bbb-z-y-x-123[abxyz]
    a-b-c-d-e-f-g-h-987[abcde]
    not-a-real-room-404[oarel]
    totally-real-room-200[decoy]",
        );
        assert_eq!(actual, 1514);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
