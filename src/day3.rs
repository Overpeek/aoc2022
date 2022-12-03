use std::collections::{HashMap, HashSet};

//

fn main(input: &str) -> impl crate::Results {
    let iter = input.lines().map(|s| s.as_bytes());

    let p1 = iter
        .clone()
        .map(|s| s.split_at(s.len() / 2))
        .flat_map(|(a, b)| {
            let map = a.iter().copied().collect::<HashSet<_>>();
            b.iter().copied().find(move |c| map.contains(c))
        })
        .map(priority)
        .sum::<u32>();

    let p2 = iter
        .map(|s| s.iter().copied().collect::<HashSet<_>>())
        .array_chunks::<3>()
        .flat_map(|s| {
            let mut map = HashMap::<_, u8>::new();
            s.iter().flat_map(|s| s.iter()).for_each(|c| {
                *map.entry(c).or_default() += 1;
            });
            map.into_iter().find(|(_, v)| *v == 3).map(|(k, _)| *k)
        })
        .map(priority)
        .sum::<u32>();

    (p1, p2)
}

fn priority(c: u8) -> u32 {
    (1 + c - if c > b'Z' { b'a' } else { b'A' - 26 }) as u32
}

crate::bp!(3);
