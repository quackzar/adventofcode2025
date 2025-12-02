

fn solve1(input: &str) -> u64 {
    let input : Vec<_> = input.trim().split(',')
        .map(|range| {
            let Some((fst, snd)) = range.split_once('-') else {
                panic!("Could not parse: {range}");
            };
            let fst : u64 = fst.parse().unwrap();
            let snd : u64 = snd.trim_end().parse().unwrap();
            (fst, snd)
        }).collect();


    let mut sum = 0;
    for (start, end) in input {

        for id in start..=end {
            let id = id.to_string().into_bytes();
            let n =  id.len();
            let left = &id[..n/2];
            let right = &id[n/2..];
            let left = str::from_utf8(left).unwrap();
            let right = str::from_utf8(right).unwrap();
            if left == right {
                let id : u64 = str::from_utf8(&id).unwrap().parse().unwrap();
                sum += id;
            }

        }

    }

    sum
}

fn solve2(input: &str) -> u64 {
    let input : Vec<_> = input.trim().split(',')
        .map(|range| {
            let Some((fst, snd)) = range.split_once('-') else {
                panic!("Could not parse: {range}");
            };
            let fst : u64 = fst.parse().unwrap();
            let snd : u64 = snd.trim_end().parse().unwrap();
            (fst, snd)
        }).collect();


    let mut sum = 0;
    for (start, end) in input {

        for id in start..=end {
            let id = id.to_string().into_bytes();
            let n =  id.len();
            'outer: for k in (1..=n/2).rev() {
                let mut segments = id.chunks(k);
                let first_segment: &[u8] = segments.next().unwrap();
                for segment in segments {
                    if first_segment != segment {
                        continue 'outer;
                    }
                }
                let id : u64 = str::from_utf8(&id).unwrap().parse().unwrap();
                sum += id;
                break 'outer;
            }

        }

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
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124\
";

    #[test]
    fn part1() {
        assert_eq!(solve1(&INPUT), 1227775554);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(&INPUT), 4174379265);
    }
}
