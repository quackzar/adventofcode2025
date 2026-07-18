use std::collections::btree_map::Range;

use itertools::Itertools;
use rustc_hash::FxHashSet;

fn solve1(input: &str) -> u64 {
    let red_tiles : FxHashSet<(u32, u32)> = input.lines()
        .map(|line| {
            let (x,y) = line.split_once(',').unwrap();
            let x : u32 = x.parse().unwrap();
            let y : u32 = y.parse().unwrap();
            (x,y)
        })
        .collect();


    red_tiles.iter()
        .tuple_combinations()
        .map(|(a,b)| {
            (a.0.abs_diff(b.0) + 1) as u64
                *
            (a.1.abs_diff(b.1) + 1) as u64
        }).max().unwrap()
}

fn order<T: PartialOrd>(a: T, b: T) -> (T,T) {
    if a < b {
        (a,b)
    } else {
        (b,a)
    }
}


fn solve2(input: &str) -> u64 {
    let red_tiles : Vec<(u32, u32)> = input.lines()
        .map(|line| {
            let (x,y) = line.split_once(',').unwrap();
            let x : u32 = x.parse().unwrap();
            let y : u32 = y.parse().unwrap();
            (x,y)
        })
        .collect();

    let mut green_tiles = FxHashSet::default();

    // build walls
    for (t1, t2) in red_tiles.iter().circular_tuple_windows() {
        if t1.0 == t2.0 {
            let (a,b) = order(t1.1, t2.1);
            let x = t1.0;
            green_tiles.extend((a..=b).map(|y| (x,y)))
        } else {
            let (a,b) = order(t1.0, t2.0);
            let y = t1.1;
            green_tiles.extend((a..=b).map(|x| (x,y)))
        }
    }

    // coloring inside
    // candidate might be wrong, who knows
    let good_candidate = (red_tiles[0].0+1, red_tiles[0].1+1);
    let mut work = vec![good_candidate];
    // Too slow :(
    while let Some(t) = work.pop() {
        for t in [
            (t.0-1, t.1),
            (t.0+1, t.1),
            (t.0, t.1-1),
            (t.0, t.1+1),
        ] {
            if !green_tiles.contains(&t) {
                green_tiles.insert(t);
                work.push(t)
            }
        }
    }


    red_tiles.iter()
        .tuple_combinations()
        .filter(|(a,b)| {
            let c = (a.0, b.1);
            let d = (b.0, a.1);
            green_tiles.contains(&c) &&
            green_tiles.contains(&d)
        })
        .map(|(a,b)| {
            (a.0.abs_diff(b.0) + 1) as u64
                *
            (a.1.abs_diff(b.1) + 1) as u64
        }).max().unwrap()
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
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn part1() {
        assert_eq!(solve1(INPUT), 50);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(INPUT), 24);
    }
}
