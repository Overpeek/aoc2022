fn main(input: &str) -> impl crate::Results {
    (sol(4, input), sol(14, input))
}

fn sol(n: usize, input: &str) -> usize {
    input
        .as_bytes()
        .windows(n)
        .enumerate()
        // O(n²) > O(n) here because hashing and allocating is way slower
        .find(|(_, s)| !(1..s.len()).any(|i| s[i..].contains(&s[i - 1])))
        .map(|(i, _)| i + n)
        .unwrap_or(0)
}

crate::bp!(6);
