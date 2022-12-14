fn main(input: &str) -> impl crate::Results {
    let iter = input.lines().map(|s| s.as_bytes());

    let p1 = iter
        .clone()
        .map(|s| s.split_at(s.len() / 2))
        .flat_map(|(a, b)| b.iter().copied().find(move |c| a.contains(c)))
        .map(priority)
        .sum::<u32>();

    let p2 = iter
        .array_chunks::<3>()
        .map(|s| {
            let mut map = [(0, 0); 256];
            for (i, s) in s
                .iter()
                .enumerate()
                .flat_map(|(i, s)| s.into_iter().map(move |c| (i, *c)))
            {
                let (entry, already_in) = &mut map[s as usize];

                if *already_in & (1 << i) != 0 {
                    continue;
                }

                if *entry == 2 {
                    return s;
                }

                *already_in |= 1 << i;
                *entry += 1;
            }
            unreachable!()
        })
        .map(priority)
        .sum::<u32>();

    (p1, p2)
}

fn priority(c: u8) -> u32 {
    (1 + c - if c > b'Z' { b'a' } else { b'A' - 26 }) as u32
}

crate::bp!(3);
