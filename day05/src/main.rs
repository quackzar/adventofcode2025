#![feature(iterator_try_collect)]
use std::collections::{BTreeSet, VecDeque};

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
        if line.is_empty() {
            break;
        };
        let (begin, end) = line.split_once('-').unwrap();
        let begin: u64 = begin.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        ranges.insert((begin, end));
    }

    let ids: Vec<u64> = input.map(str::parse).try_collect().unwrap();

    let mut fresh = 0;
    for id in ids {
        for &(begin, end) in ranges.iter() {
            if begin <= id && id <= end {
                fresh += 1;
                break;
            }
        }
    }

    fresh
}

fn solve2(input: &str) -> u64 {
    let mut ranges = Vec::new();
    let mut input = input.lines();
    for line in input.by_ref() {
        if line.is_empty() {
            break;
        };
        let (begin, end) = line.split_once('-').unwrap();
        let begin: u64 = begin.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        ranges.push((begin, end));
    }

    // optimize
    let mut disjoint = Vec::new();
    ranges.sort_by_key(|&(begin, _)| begin);
    let mut ranges: VecDeque<_> = ranges.into();
    loop {
        // invariant: first pair is always starts before second pair
        let (f0, t0) = ranges.pop_front().unwrap();
        let Some((f1, t1)) = ranges.pop_front() else {
            // none left
            disjoint.push((f0, t0));
            break;
        };
        if t1 <= t0 {
            // the second range is irrelevant as it is shorter
            ranges.push_front((f0, t0));
        } else if f1 <= t0 {
            // ranges can be combined
            ranges.push_front((f0, t1));
        } else if t0 < f1 {
            // ranges are disjoint
            disjoint.push((f0, t0));
            // retry second
            ranges.push_front((f1, t1));
        }
    }

    let mut total_fresh = 0;
    for (begin, end) in disjoint {
        // add one since they are inclusive
        println!("{begin}-{end}");
        total_fresh += end - begin + 1;
    }

    total_fresh
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
        assert_eq!(solve1(INPUT), 3);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(INPUT), 14);
    }
}
