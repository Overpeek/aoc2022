fn main(input: &str) -> (i32, i32) {
    let iter = input
        .lines()
        .filter_map(|s| -> Option<[u8; 3]> { s.as_bytes().try_into().ok() })
        .map(|[a, _, b]| [a - 65, b - 88]);

    let p1 = iter
        .clone()
        .map(|[a, b]| (1 - (a as i8 - b as i8)).rem_euclid(3) as u8 * 3 + b + 1)
        .map(|i| i as i32)
        .sum::<i32>();

    let p2 = iter
        .map(|[a, b]| b * 3 + ((a + b) as i8 - 1).rem_euclid(3) as u8 + 1)
        .map(|i| i as i32)
        .sum::<i32>();

    (p1, p2)
}

crate::bp!(2);
