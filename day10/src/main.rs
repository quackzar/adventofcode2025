#![feature(uint_gather_scatter_bits)]

use foldhash::{HashMap, HashSet};
use std::collections::VecDeque;
use arrayvec::ArrayVec;
use itertools::Itertools;
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

const CAP : usize = 16;

#[derive(Debug)]
struct Machine {
    n: u8,
    lights: u32,
    buttons: Vec<u32>,
    joltage: ArrayVec<u16, CAP>,
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
        .into_par_iter()
        .enumerate()
        .map(|(num, machine)| {
            let initial_joltage: ArrayVec<_, CAP> = machine.joltage.iter().copied().map(|j| j as i16).collect();
            let buttons: Vec<_> = machine.buttons.iter()
                .map(|b| {
                    let mut button: ArrayVec<_, CAP> = ArrayVec::new();
                    for i in 0..machine.n {
                        if b.extract_bits(1 << i) != 0 {
                            button.push(i);
                        }
                    }
                    button
                }).collect();

            let mut stack = VecDeque::new();

            let set = ArrayVec::<_, CAP>::from_iter(0..buttons.len() as u8);

            let mut button_mat = HashMap::default();
            let n = machine.n;
            let m = buttons.len() as u8;
            for j in 0..m {
                for i in 0..n {
                    if buttons[j as usize].contains(&i) {
                        button_mat.insert((i,j), 1i16);
                    }
                }
            }

            let weights = ArrayVec::from([0; CAP]);
            stack.push_back((weights.clone(), 0));

            let mut tried = HashSet::default();
            let mut attempts = 0;

            'outer: loop {
                attempts += 1;
                let (weights, depth) = stack.pop_front().unwrap();

                let mut joltage = initial_joltage.clone();
                for i in 0..n {
                    for j in 0..m {
                        joltage[i as usize] -= button_mat.get(&(i,j)).copied().unwrap_or_default() * weights[j as usize];
                    }
                }

                if joltage.iter().all(|j| *j == 0) {
                    let x = stack.len();
                    println!("Machine {num} has succeded after {attempts} attempts {x} left on stack [depth: {depth}]");
                    break 'outer depth;
                }

                let mut set = set.clone();
                set.retain(|b| {
                    // Remove bad buttons
                    !buttons[*b as usize].iter().any(|i| joltage[*i as usize] == 0)
                });
                set.retain(|b| {
                    // Remove unhelpful buttons
                    joltage.iter().enumerate().any(|(i,_)| {
                        buttons[*b as usize].contains(&(i as u8))
                    })
                });

                // Sort by most valuable button
                set.sort_unstable_by(|x, y| {
                    let xs : u32 = buttons[*x as usize].iter().map(|i| joltage[*i as usize] as u32).sum();
                    let ys : u32 = buttons[*y as usize].iter().map(|i| joltage[*i as usize] as u32).sum();
                    xs.cmp(&ys)
                });

                for next in set.iter() {
                    let w : i16 = buttons[*next as usize].iter().map(|i| joltage[*i as usize]).min().unwrap();
                    let mut weights = weights.clone();
                    weights[*next as usize] += w;
                    let depth = depth + (w as u32);
                    if tried.insert(weights.clone()) {
                        stack.push_front((weights, depth));
                    }
                }

                for next in set.iter().rev() {
                    let w : i16 = buttons[*next as usize].iter().map(|i| joltage[*i as usize]).min().unwrap();
                    let w = w / 2 + 1;
                    let mut weights = weights.clone();
                    weights[*next as usize] += w;
                    let depth = depth + (w as u32);
                    if tried.insert(weights.clone()) {
                        stack.push_back((weights, depth));
                    }
                }
            }
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
