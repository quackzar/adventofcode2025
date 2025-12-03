#![feature(explicit_tail_calls)]
#![allow(incomplete_features)]
#![allow(clippy::needless_range_loop)]

fn solve1(input: &str) -> u64 {
    let input = input.lines()
        .map(str::as_bytes);

    let mut sum = 0;
    for line in input {
        let n = line.len();
        let mut bank = [0u8;2];

        let mut max = 0;
        for i in 0..n {
            bank[0] = line[i];
            for j in i+1..n {
                bank[1] = line[j];

                let bank: u8 = str::from_utf8(&bank).unwrap().parse().unwrap();
                if bank > max {
                    max = bank;
                }
            }
        }
        sum += max as u64;
    }

    sum
}

fn solve2(input: &str) -> u64 {
    let input = input.lines()
        .map(str::as_bytes);

    let mut sum = 0;
    for line in input {
        fn h(depth: usize, batteries: &[u8], bank: &mut [u8; 12]) -> u64 {
            if depth == 12 || batteries.is_empty() {
                str::from_utf8(bank).unwrap().parse().unwrap()
            } else {
                let (n, digit) = batteries.iter().enumerate().rev()
                    .skip(11-depth)
                    .max_by_key(|(_, digit)| *digit).unwrap();
                bank[depth] = *digit;
                become h( depth+1, &batteries[n+1..], bank)
            }
        }

        let mut bank = [b'0';12];
        let res = h( 0, line, &mut bank);
        sum += res;
    }

    sum
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
987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn part1() {
        assert_eq!(solve1(&INPUT), 357);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(&INPUT), 3121910778619);
    }
}
