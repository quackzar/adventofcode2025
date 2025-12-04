#![feature(ascii_char)]
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Field { Paper, Empty }

fn parse(input: &str) -> HashMap<(isize, isize), Field> {
    input.lines()
        .map(str::bytes)
        .enumerate()
        .flat_map(|(i, b)|
            b.enumerate()
            .map(move |(j, c)| ((i as isize, j as isize), c))
        )
        .map(|(pos, field)| match field {
            b'@' => (pos, Field::Paper),
            b'.' => (pos, Field::Empty),
            _ => {
                let field = field.as_ascii().unwrap();
                panic!("unknown: '{field:?}'")
            }
        })
        .collect()
}

fn solve1(input: &str) -> u64 {
    let diagram = parse(input);
    let rolls = diagram.iter().filter(|(_,x)| **x == Field::Paper)
        .map(|(pos, _)| pos)
        .cloned();

    let mut total = 0;

    for (x,y) in rolls {
        let mut neighbours: u32 = 0;
        for i in [x-1, x, x+1] {
            for j in [y-1, y, y+1] {
                if (x,y) == (i,j) { continue } // skip self
                if let Some(Field::Paper) = diagram.get(&(i,j)) { neighbours += 1 }
            }
        }
        if neighbours < 4 {
            total += 1;
        }
    }

    total
}

fn solve2(input: &str) -> u64 {
    let diagram = parse(input);
    let mut rolls: HashSet<_> = diagram.iter().filter(|(_,x)| **x == Field::Paper)
        .map(|(pos, _)| pos)
        .cloned()
        .collect();

    let rolls_beginning = rolls.len();

    loop {
        let image = rolls.clone();
        rolls.retain(|pos| {
            let (x,y) = *pos;
            let mut neighbours: u32 = 0;
            for i in [x-1, x, x+1] {
                for j in [y-1, y, y+1] {
                    if (x,y) == (i,j) { continue } // skip self
                    if image.contains(&(i,j)) {
                        neighbours += 1;
                    }
                }
            }
            neighbours >= 4
        });

        if rolls.len() < image.len() {
            let m = image.len() - rolls.len();
            println!("removed {m}");
            continue
        } else {
            break
        }
    }

    (rolls_beginning - rolls.len()) as u64
}

fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let res = solve1(&input);
    println!("Solution 1: {res}");
    let res = solve2(&input);
    println!("Solution 2: {res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn part1() {
        assert_eq!(solve1(&INPUT), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(&INPUT), 43);
    }
}
