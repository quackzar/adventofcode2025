use std::{cell::RefCell, rc::Rc};

use rustc_hash::{FxHashMap, FxHashSet};
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

fn solve1(input: &str) -> u64 {
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

    // sparse graph of connections
    type SharedSet = Rc<RefCell<FxHashSet<Point>>>;
    let mut net: FxHashMap<Point, SharedSet> = FxHashMap::default();

    jbs0.cartesian_product(jbs1)
        .sorted_unstable_by_key(|(jb0, jb1)| jb0.dist(jb1))
        .skip(n) // skip the ones where we connect ourselves.
        .map(|(jb0, jb1)| {
            if jb0 < jb1 { (jb0, jb1) } else { (jb1, jb0) }
        })
        .unique()
        .take(10)
        .for_each(|(&jb0, &jb1)| {
            let len = jb0.dist(&jb1);
            println!("{jb0:?} and {jb1:?}: {len}");
            let pair = net.get_disjoint_mut([&jb0, &jb1]);
            match pair {
                [Some(set0), Some(set1)] => {
                    // connect all
                    if !Rc::ptr_eq(set0, set1) {
                        let set: FxHashSet<_>  = set0.borrow_mut().union(&set1.borrow_mut()).copied().collect();
                        let set = Rc::new(RefCell::new(set));
                        *set0 = set.clone();
                        *set1 = set;
                    }
                }
                [Some(set), None] => {
                    set.borrow_mut().insert(jb1);
                    set.borrow_mut().insert(jb0);
                    let ptr = set.clone();
                    net.entry(jb1).or_insert_with(|| ptr);
                }
                [None, Some(set)] => {
                    set.borrow_mut().insert(jb1);
                    set.borrow_mut().insert(jb0);
                    let ptr = set.clone();
                    net.entry(jb0).or_insert_with(|| ptr);
                }
                _ => {
                    let set = [jb0, jb1];
                    let set = Rc::new(RefCell::new(FxHashSet::from_iter(set)));
                    net.entry(jb0).or_insert_with(||set.clone());
                    net.entry(jb1).or_insert_with(||set);
                },
            }
        });


    for (key, val) in net.iter() {
        let len = val.borrow().len();
        let id = Rc::as_ptr(val) as usize;
        println!("{key:?} with set #{id:x} length {len}");
    }

    net.values()
        .unique_by(|set| Rc::as_ptr(set))
        .map(|set| set.borrow_mut().len() as u64)
        .map(|len| len.max(1))
        .sorted()
        .rev()
        .take(3)
        .inspect(|x| println!("{x}"))
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
