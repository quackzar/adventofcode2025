#![feature(ascii_char_variants)]
#![feature(ascii_char)]

use std::ascii::Char;



fn parse(input: &str) -> Vec<i16> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let line = line.as_ascii().unwrap();
            let dir = line[0];
            let num: i16 = line[1..].as_str().parse().unwrap();

            match dir {
                Char::CapitalL => -num,
                Char::CapitalR => num,
                _ => panic!("Not supposed to happen: dir {dir}")
            }
        }).collect()
}

fn solve1(input: &[i16]) -> u32 {
    let mut dial: i16 = 50;
    let mut zeros = 0;
    for rot in input {
        dial += *rot;
        dial = dial.rem_euclid(100);

        if dial == 0 {
            zeros += 1;
        }
    }
    zeros
}


fn solve2(input: &[i16]) -> i16 {
    let mut dial: i16 = 50;
    let mut zeros = 0;
    for &rot in input {
        zeros += rot.abs() / 100;
        let was_zero = dial == 0;
        dial += rot % 100;

        if (dial <= 0) && !was_zero || dial >= 100 {
            zeros += 1;
        }

        dial = dial.rem_euclid(100);
    }
    zeros
}

fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let input = parse(&input);
    let res = solve1(&input);
    println!("Solution 1: {res}");
    let res = solve2(&input);
    println!("Solution 2: {res}");
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn part1() {
        let input = parse(INPUT);
        assert_eq!(solve1(&parse(INPUT)), 3);
    }

    #[test]
    fn part2() {
        let input = parse(INPUT);
        assert_eq!(solve2(&parse(INPUT)), 6);
    }
}
