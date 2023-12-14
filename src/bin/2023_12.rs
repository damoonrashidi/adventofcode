fn main() {
    let input = include_str!("../../inputs/2023/12.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (list, nums_str) = line.split_once(' ').unwrap();
            let nums: Vec<u32> = nums_str.split(',').map(|c| c.parse().unwrap()).collect();
            let chars: Vec<char> = list.chars().collect();
            valid_permutations(&chars, &nums)
        })
        .sum()
}

fn puzzle_two(_input: &str) -> u32 {
    0
}

fn valid_permutations(_list: &[char], _nums: &[u32]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::puzzle_one;

    #[test]
    fn test_puzzle_one() {
        assert_eq!(puzzle_one(".??..??...?##. 1,1,3"), 4);
        assert_eq!(puzzle_one("???.### 1,1,3"), 1);
        assert_eq!(puzzle_one("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(puzzle_one("????.#...#... 4,1,1"), 1);
        assert_eq!(puzzle_one("????.######..#####. 1,6,5"), 4);
        assert_eq!(puzzle_one("?###???????? 3,2,1"), 10);
    }
}
