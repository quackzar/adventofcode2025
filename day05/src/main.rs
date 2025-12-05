#![feature(iterator_try_collect)]
use std::collections::BTreeSet;

fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let res = solve1(&input);
    println!("Solution 1: {res}");
    let res = solve2(&input);
    println!("Solution 2: {res}");
}


fn solve1(input: &str) -> u64 {
    let mut ranges = BTreeSet::new();
    let mut input = input.lines();
    for line in input.by_ref() {
        if line == "\n" { break }
        let (begin, end) = line.split_once('-').unwrap();
        let begin: u32 = begin.parse().unwrap();
        let end: u32 = end.parse().unwrap();
        ranges.insert((begin, end));
    }

    let ids: Vec<u32> = input.map(str::parse)
        .try_collect().unwrap();




    todo!()
}

fn solve2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn part1() {
        assert_eq!(solve1(INPUT), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(INPUT), 43);
    }
}
