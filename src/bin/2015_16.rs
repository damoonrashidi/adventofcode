fn main() {
    let input = include_str!("../../inputs/2015/16.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let ticker = std::collections::HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    for (i, line) in input.lines().enumerate() {
        let (_, split) = line.split_once(": ").unwrap();
        let attrs = split
            .split(", ")
            .map(|chrs| {
                let (attr, val) = chrs.split_once(": ").unwrap();
                (attr, val.parse::<usize>().unwrap())
            })
            .collect::<Vec<(&str, usize)>>();

        let mut correct = true;
        for (attr, val) in &attrs {
            if val != ticker.get(attr).unwrap() {
                correct = false;
                break;
            }
        }

        if correct {
            println!("{attrs:?}");
            return i + 1;
        }
    }

    0
}

fn puzzle_two(input: &str) -> usize {
    {
        let ticker = std::collections::HashMap::from([
            ("children", 3),
            ("cats", 7),
            ("samoyeds", 2),
            ("pomeranians", 3),
            ("akitas", 0),
            ("vizslas", 0),
            ("goldfish", 5),
            ("trees", 3),
            ("cars", 2),
            ("perfumes", 1),
        ]);

        for (i, line) in input.lines().enumerate() {
            let (_, split) = line.split_once(": ").unwrap();
            let attrs = split
                .split(", ")
                .map(|chrs| {
                    let (attr, val) = chrs.split_once(": ").unwrap();
                    (attr, val.parse::<usize>().unwrap())
                })
                .collect::<Vec<(&str, usize)>>();

            let mut correct = true;
            for (attr, val) in attrs {
                let asd = *ticker.get(attr).unwrap();

                if attr == "cats" || attr == "trees" {
                    if val <= asd {
                        correct = false;
                        break;
                    }
                } else if attr == "pomeranians" || attr == "goldfish" {
                    if val >= asd {
                        correct = false;
                        break;
                    }
                } else if val != asd {
                    correct = false;
                    break;
                }
            }
            if correct {
                return i + 1;
            }
        }
    }

    0
}
