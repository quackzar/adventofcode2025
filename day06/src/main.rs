fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let res = solve1(&input);
    println!("Solution 1: {res}");
    let res = solve2(&input);
    println!("Solution 2: {res}");
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

fn solve1(input: &str) -> u64 {
    let mut input = input.lines().rev();

    let operators: Vec<_> = input
        .by_ref()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| match x.trim() {
            "*" => Operator::Multiply,
            "+" => Operator::Add,
            _ => panic!("unknown operator '{x}'"),
        })
        .collect();

    let mut reg: Vec<_> = operators
        .iter()
        .map(|op| match op {
            Operator::Add => 0,
            Operator::Multiply => 1,
        })
        .collect();

    for line in input.map(str::split_whitespace) {
        line.map(str::parse::<u64>)
            .map(Result::unwrap)
            .inspect(|x| print!("{x} "))
            .enumerate()
            .for_each(|(i, num)| match operators[i] {
                Operator::Add => reg[i] += num,
                Operator::Multiply => reg[i] *= num,
            });
        println!()
    }

    reg.iter().sum()
}

fn solve2(input: &str) -> u64 {
    let input = input.as_bytes();

    let (n, _) = input
        .iter()
        .enumerate()
        .find(|(_, b)| **b == b'\n')
        .unwrap();

    let m = input.iter().filter(|b| **b == b'\n').count();
    let map = |i: usize, j: usize| -> u8 { input[i + j * (n + 1)] };

    let mut res = 0;
    let mut num = Vec::new();
    let mut stack: Vec<u64> = Vec::new();
    for i in (0..n).rev() {
        let mut operator = None;
        num.clear();
        for j in 0..m {
            match map(i, j) {
                b' ' => continue,
                b'+' => {
                    operator = Some(Operator::Add);
                    break;
                }
                b'*' => {
                    operator = Some(Operator::Multiply);
                    break;
                }
                c @ b'0'..=b'9' => {
                    num.push(c);
                }
                b'\n' => break,
                c => {
                    let c = c as char;
                    panic!("unknown char: '{c}'")
                }
            }
        }
        if let Ok(num) = str::from_utf8(&num).unwrap().parse() {
            stack.push(num);
        }
        match operator {
            Some(Operator::Add) => {
                res += stack.drain(..).sum::<u64>();
            }
            Some(Operator::Multiply) => {
                res += stack.drain(..).product::<u64>();
            }
            None => (),
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + 
";

    #[test]
    fn part1() {
        assert_eq!(solve1(INPUT), 4277556);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(INPUT), 3263827);
    }
}
