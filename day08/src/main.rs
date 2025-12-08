#![feature(iterator_try_collect)]
use std::{collections::{HashMap, HashSet}, hash::Hash};

use itertools::Itertools;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u64,
    y: u64,
    z: u64
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist(&Default::default()).partial_cmp(&other.dist(&Default::default()))
    }
}

impl Point {
    fn dist(&self, other: &Self) -> u64 {
        let dist_sq = self.x.abs_diff(other.x).pow(2) +
        self.y.abs_diff(other.y).pow(2) +
        self.z.abs_diff(other.z).pow(2);
        dist_sq.isqrt()
    }
}

fn solve1(input: &str) -> u64 {
    let mut junction_boxes : HashSet<_> = input.lines()
        .map(|s| {
            let [x,y,z] = s.split(',').take(3)
                .map(str::parse::<u64>)
                .collect::<Result<Vec<_>, _>>()
                .unwrap()[..] else { unreachable!() };
            Point { x, y, z }
        }).collect();

    let mut circuits: HashMap<Point, HashSet<Point>> = HashMap::new();
    for _ in 0..10 {
        let Some((j1,j2)) = junction_boxes.iter()
            .cartesian_product(junction_boxes.iter())
            .filter(|(j1,j2)| {
                let circuit = circuits.entry(**j1).or_default();
                !circuit.contains(*j2)
            })
            .min_by_key(|(j1,j2)| j1.dist(j2) ) else { break };

        circuits.entry(*j1).or_default().insert(*j2);
        circuits.entry(*j2).or_default().insert(*j1);
    }

    dbg!(&circuits);

    circuits.values()
        .map(HashSet::len)
        .map(|x| x > 0)
        .map(u64::try_from)
        .map(Result::unwrap)
        .product()
}

fn solve2(input: &str) -> u64 { todo!() }


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
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn part1() {
        assert_eq!(solve1(INPUT), 40);
    }

    // #[test]
    // fn part2() {
    //     assert_eq!(solve2(INPUT), 40);
    // }
}
