#![allow(incomplete_features)]
#![feature(explicit_tail_calls)]
#![feature(type_alias_impl_trait)]
use std::{
    collections::{HashMap, HashSet},
    mem,
};

fn solve1(input: &str) -> u64 {
    let input = input.as_bytes();
    let (n, _) = input
        .iter()
        .enumerate()
        .find(|(_, b)| **b == b'\n')
        .unwrap();

    let m = input.iter().filter(|b| **b == b'\n').count();
    let map = |i: usize, j: usize| -> u8 { input[i + j * (n + 1)] };

    let mut beams = HashSet::new();
    for i in 0..n {
        if map(i, 0) == b'S' {
            beams.insert(i);
        }
    }

    let mut splits = 0;
    for j in 1..m {
        let mut next = HashSet::new();
        for i in beams.drain() {
            match map(i, j) {
                b'.' => {
                    next.insert(i);
                }
                b'^' => {
                    splits += 1;
                    next.insert(i - 1);
                    next.insert(i + 1);
                }
                x => {
                    let x = x as char;
                    panic!("unhandled case '{x}'")
                }
            };
        }

        for i in 0..n {
            let x = map(i, j) as char;
            let x = if next.contains(&i) { '|' } else { x };
            print!("{x}")
        }
        println!();
        mem::swap(&mut next, &mut beams)
    }

    splits
}

fn solve2(input: &str) -> u64 {
    let input = input.as_bytes();
    let (n, _) = input
        .iter()
        .enumerate()
        .find(|(_, b)| **b == b'\n')
        .unwrap();

    let m = input.iter().filter(|b| **b == b'\n').count();
    let mut map = |i: usize, j: usize| -> Option<u8> {
        let idx = i + j * (n + 1);
        input.get(idx).copied()
    };

    type Position = (usize, usize);
    fn search<M>(mem: &mut HashMap<Position, u64>, map: &mut M, i: usize, j: usize) -> u64
    where
        M: FnMut(usize, usize) -> Option<u8>,
    {
        if let Some(&memorized) = mem.get(&(i, j)) {
            return memorized;
        }
        let splits = match map(i, j) {
            Some(b'.') => search(mem, map, i, j + 1),
            Some(b'^') => search(mem, map, i - 1, j + 1) + search(mem, map, i + 1, j + 1),
            None => 1,
            Some(x) => {
                let x = x as char;
                panic!("unhandled case '{x}'")
            }
        };
        mem.insert((i, j), splits);
        splits
    }

    let i = (0..m).find(|&i| map(i, 0).unwrap() == b'S').unwrap();
    let mut mem = HashMap::new();
    mem.insert((i, 0), 0);
    search(&mut mem, &mut map, i, 1)
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
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn part1() {
        assert_eq!(solve1(INPUT), 21);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(INPUT), 40);
    }
}
