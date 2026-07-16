#![feature(uint_gather_scatter_bits)]

use foldhash::HashSet;
use std::collections::{BTreeSet, VecDeque};

use arrayvec::ArrayVec;
use itertools::{Itertools, multizip};
use nalgebra::{DMatrix, linalg};
use rayon::prelude::*;

fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim_start();
    let res = solve1(input);
    println!("Solution 1: {res}");
    let res = solve2(input);
    println!("Solution 2: {res}");
}

#[derive(Debug)]
struct Machine {
    n: u8,
    lights: u32,
    buttons: Vec<u32>,
    joltage: ArrayVec<u16, 16>,
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for i in 0..self.n {
            let b = self.lights.extract_bits(1 << i);
            if b != 0 {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        write!(f, "] ")?;

        for wiring in self.buttons.iter() {
            write!(f, "(")?;
            let mut is_first = true;
            for i in 0..self.n {
                if wiring.extract_bits(1 << i) != 0 {
                    if is_first {
                        write!(f, "{i}")?;
                        is_first = false;
                    } else {
                        write!(f, ",{i}")?;
                    }
                }
            }
            write!(f, ") ")?;
        }

        write!(f, "{{")?;
        let mut iter = self.joltage.iter();

        if let Some(jolt) = iter.next() {
            write!(f, "{jolt}")?;
        }
        for joltage in iter {
            write!(f, ",{joltage}")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|s| {
            let mut iter = s.chars().peekable();
            iter.next(); // skip '['
            let mut lights: u32 = 0;
            let mut i: u8 = 0;
            while *iter.peek().expect("end of lights section") != ']' {
                let is_on = iter.next().unwrap() != '.';
                if is_on {
                    lights |= 1 << i;
                }
                i += 1;
            }

            let n = i;
            iter.next(); // skip ']'

            let mut buttons = Vec::new();
            let mut num = String::new();
            let mut wiring: u32 = 0;
            while let Some(c) = iter.next_if(|c| *c != '{') {
                match c {
                    '0'..='9' => {
                        num.push(c);
                    }
                    ',' => {
                        let x: u8 = num.parse().unwrap();
                        wiring |= 1 << x;
                        num.clear();
                    }
                    ')' => {
                        let x: u8 = num.parse().unwrap();
                        wiring |= 1 << x;
                        buttons.push(wiring);
                        num.clear();
                        wiring = 0;
                    }
                    _ => {}
                }
            }

            num.clear();
            let mut joltage = ArrayVec::new();
            while let Some(c) = iter.next_if(|c| *c != '}') {
                match c {
                    '0'..='9' => {
                        num.push(c);
                    }
                    ',' => {
                        let x: u16 = num.parse().unwrap_or_else(|e| {
                            eprintln!("failed parsing number: {num}");
                            panic!("{e}")
                        });
                        joltage.push(x);
                        num.clear();
                    }
                    _ => {}
                }
            }
            let x: u16 = num.parse().unwrap();
            joltage.push(x);
            Machine {
                n,
                lights,
                buttons,
                joltage,
            }
        })
        .collect()
}

fn solve1(input: &str) -> u32 {
    let machines = parse(input);
    machines
        .into_par_iter()
        .map(|machine| {
            let lights = machine.lights;
            let mut depth = 0;
            loop {
                if machine
                    .buttons
                    .iter()
                    .combinations(depth)
                    .map(|buttons| {
                        buttons
                            .iter()
                            .copied()
                            .fold(lights, |state, &wiring| state ^ wiring)
                    })
                    .any(|state| state == 0)
                {
                    break depth as u32;
                } else {
                    depth += 1;
                }
            }
        })
        .sum()
}

fn solve2(input: &str) -> u32 {
    let machines = parse(input);
    machines
        .into_iter()
        .map(|machine| {
            let joltage: ArrayVec<_, 16> = machine.joltage.clone();
            let mut buttons: Vec<_> = machine
                .buttons
                .iter()
                .map(|b| {
                    let mut button: ArrayVec<_, 16> = ArrayVec::new();
                    for i in 0..machine.n {
                        if b.extract_bits(1 << i) != 0 {
                            button.push(i);
                        }
                    }
                    button
                })
                .collect();

            println!("{buttons:?}");
            let mut stack = VecDeque::new();

            let mut set = ArrayVec::<_, 16>::from_iter(0..buttons.len());
            set.retain(|b| {
                joltage.iter().enumerate().any(|(i, j)| {
                    if *j > 0 {
                        buttons[*b].contains(&(i as u8))
                    } else {
                        false
                    }
                })
            });
            for b in 0..buttons.len() {
                stack.push_back((joltage.clone(), b, 0, set.clone()));
            }

            let m = buttons.len();
            let n = machine.n as usize;

            type c32 = nalgebra::Complex<f32>;

            let mut big_button = DMatrix::<c32>::zeros(n, m);

            for (j, b) in buttons.iter().enumerate() {
                for i in b {
                    let i = *i as usize;
                    big_button[(i, j)] = c32::ONE;
                }
            }
            let joltage = nalgebra::DVector::from_iterator(
                n,
                joltage.iter().copied().map(|x| c32::new(x as f32, 0.)),
            );

            let decomp = big_button.svd(true, true);
            let weights = decomp.solve(&joltage, 0.001).unwrap();
            println!("{weights:?}");

            weights.sum().re as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn part1() {
        let res = solve1(TEST_INPUT);
        assert_eq!(res, 7);
    }

    #[test]
    fn part2() {
        let res = solve2(TEST_INPUT);
        assert_eq!(res, 33);
    }
}
