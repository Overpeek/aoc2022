fn main(input: &str) -> impl crate::Results {
    let iter = input
        .lines()
        .filter_map(|s| -> Option<[u8; 3]> { s.as_bytes().try_into().ok() })
        .map(|[a, _, b]| [a as i32 - 65, b as i32 - 88]);

    let p1 = iter
        .clone()
        .map(|[a, b]| (1 - a + b).rem_euclid(3) as u32 * 3 + b as u32 + 1)
        .sum::<u32>();

    let p2 = iter
        .map(|[a, b]| b as u32 * 3 + (a + b - 1).rem_euclid(3) as u32 + 1)
        .sum::<u32>();

    (p1, p2)
}

crate::bp!(2);
