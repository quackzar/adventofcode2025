use std::collections::BTreeMap;

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

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Name([u8; 3]);

const YOU: Name = Name(*b"you");
const OUT: Name = Name(*b"out");
const SVR: Name = Name(*b"svr");
const DAC: Name = Name(*b"dac");
const FFT: Name = Name(*b"fft");

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = str::from_utf8(&self.0).unwrap();
        write!(f, "{s}")
    }
}

fn parse(input: &str) -> BTreeMap<Name, Vec<Name>> {
    input.lines()
        .map(|s| {
            let (from, rest) = s.split_once(":").unwrap();
            let from: [u8; 3] = from.as_bytes().try_into().unwrap();
            let from = Name(from);

            type Array = [u8; 3];
            let to: Vec<Name> = rest.split(" ")
                .filter_map(|s| Array::try_from(s.as_bytes()).ok())
                .map(Name)
                .collect();
            (from, to)
        }).collect()
}

fn solve1(input: &str) -> u32 {
    let map = parse(input);

    let mut paths = Vec::new();
    let mut stack = Vec::new();
    stack.push((YOU, vec![]));

    while let Some((curr, mut path)) = stack.pop() {
        path.push(curr);
        if curr == OUT {
            paths.push(path);
            continue;
        }
        let devices = map.get(&curr).unwrap();
        for device in devices {
            stack.push((*device, path.clone()))
        }
    }
    paths.len() as u32
}

fn solve2(input: &str) -> u32 {
    let map = parse(input);

    let mut paths = Vec::new();
    let mut stack = Vec::new();
    stack.push((SVR, vec![]));

    // TODO:
    //
    // this is a DAG.
    //
    // 1. Find FFT first
    // 2. Then find DAC
    // 3. Then find OUT


    while let Some((curr, mut path)) = stack.pop() {
        path.push(curr);
        if curr == OUT {
            paths.push(path);
            continue;
        }

        if let Some(devices) = map.get(&curr) {
            for device in devices {
                stack.push((*device, path.clone()))
            }
        }
    }

    println!("found");
    for path in paths.iter() {
        println!("{path:?}");
    }

    paths.into_iter().filter(|path| path.contains(&DAC) && path.contains(&FFT)).count() as u32
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const EXAMPLE2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn part1() {
        assert_eq!(solve1(EXAMPLE1), 5);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(EXAMPLE2), 2);
    }
}
