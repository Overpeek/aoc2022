use std::collections::{HashMap, HashSet};

//

fn main(input: &str) -> impl crate::Results {
    let iter = input.lines().map(|s| s.as_bytes());

    let t = std::time::Instant::now();
    let p1 = iter
        .clone()
        .map(|s| s.split_at(s.len() / 2))
        .flat_map(|(a, b)| b.iter().copied().find(move |c| a.contains(c)))
        .map(priority)
        .sum::<u32>();
    println!("{:?}", t.elapsed());

    let t = std::time::Instant::now();
    let p2 = iter
        .array_chunks::<3>()
        .flat_map(|s| {
            let mut map = HashMap::<_, (u8, usize)>::new();
            for (i, s) in s
                .iter()
                .enumerate()
                .flat_map(|(i, s)| s.into_iter().map(move |c| (i, *c)))
            {
                let (entry, already_in) = map.entry(s).or_default();

                if *already_in & (1 << i) == 0 {
                    *already_in |= 1 << i;
                    *entry += 1;
                    if *entry == 3 {
                        return Some(s);
                    }
                }
            }
            map.into_iter().find(|(_, (v, _))| *v == 3).map(|(k, _)| k)
        })
        .map(priority)
        .sum::<u32>();
    println!("{:?}", t.elapsed());

    (p1, p2)
}

fn priority(c: u8) -> u32 {
    (1 + c - if c > b'Z' { b'a' } else { b'A' - 26 }) as u32
}

crate::bp!(3);
