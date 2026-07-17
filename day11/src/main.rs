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
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
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

    #[test]
    fn part1() {
        assert_eq!(solve1(EXAMPLE), 5);
    }
}
