use petgraph::{self, prelude::UnGraphMap};
use itertools::Itertools;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: u64,
    y: u64,
    z: u64
}


impl Point {
    fn dist(&self, other: &Self) -> u64 {
        let dist_sq = self.x.abs_diff(other.x).pow(2) +
        self.y.abs_diff(other.y).pow(2) +
        self.z.abs_diff(other.z).pow(2);
        dist_sq.isqrt()
    }
}

fn solve1(input: &str, size: usize) -> u64 {
    let junction_boxes : Vec<_> = input.lines()
        .map(|s| {
            let [x,y,z] = s.split(',').take(3)
                .map(str::parse::<u64>)
                .collect::<Result<Vec<_>, _>>()
                .unwrap()[..] else { unreachable!() };
            Point { x, y, z }
        }).collect();

    let n = junction_boxes.len();
    let jbs0 = junction_boxes.iter();
    let jbs1 = junction_boxes.iter();

    let mut circuit : UnGraphMap<Point, ()> = UnGraphMap::new();
    jbs0.cartesian_product(jbs1)
        .sorted_unstable_by_key(|(jb0, jb1)| jb0.dist(jb1))
        .skip(n) // skip the ones where we connect ourselves.
        .map(|(jb0, jb1)| {
            if jb0 < jb1 { (jb0, jb1) } else { (jb1, jb0) }
        })
        .unique()
        .take(size)
        .for_each(|(&jb0, &jb1)| {
            let jb0 = circuit.add_node(jb0);
            let jb1 = circuit.add_node(jb1);
            circuit.add_edge(jb0, jb1, ());
        });

        petgraph::algo::kosaraju_scc(&circuit)
            .iter()
            .map(|x| x.len() as u64)
            .sorted_unstable()
            .rev()
            .take(3)
            .inspect(|x| println!("{x}"))
            .product()
}

fn solve2(input: &str) -> u64 {
    let junction_boxes : Vec<_> = input.lines()
        .map(|s| {
            let [x,y,z] = s.split(',').take(3)
                .map(str::parse::<u64>)
                .collect::<Result<Vec<_>, _>>()
                .unwrap()[..] else { unreachable!() };
            Point { x, y, z }
        }).collect();

    let n = junction_boxes.len();
    let jbs0 = junction_boxes.iter();
    let jbs1 = junction_boxes.iter();

    let mut circuit : UnGraphMap<Point, ()> = UnGraphMap::new();
    for jb in junction_boxes.iter() {
        circuit.add_node(*jb);
    }

    for (&jb0, &jb1) in jbs0.cartesian_product(jbs1)
        .sorted_unstable_by_key(|(jb0, jb1)| jb0.dist(jb1))
        .skip(n) // skip the ones where we connect ourselves.
        {
        let jb0 = circuit.add_node(jb0);
        let jb1 = circuit.add_node(jb1);
        circuit.add_edge(jb0, jb1, ());
        if petgraph::algo::kosaraju_scc(&circuit).len() == 1 {
            println!("{jb0:?} and {jb1:?}");
            return jb0.x * jb1.x
        }
    }
    panic!("no solution found")
}


fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let res = solve1(&input, 1000);
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
        assert_eq!(solve1(INPUT, 10), 40);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(INPUT), 25272);
    }
}
