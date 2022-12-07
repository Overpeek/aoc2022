fn main(input: &str) -> impl crate::Results {
    let iter = input
        .split(|c| c == '\n' || c == ',' || c == '-')
        .filter_map(|s| s.parse::<u32>().ok())
        .array_chunks::<4>();

    let p1 = iter
        .clone()
        .filter(|[a, b, c, d]| {
            (*a..=*b).contains(c) && (*a..=*b).contains(d)
                || (*c..=*d).contains(a) && (*c..=*d).contains(b)
        })
        .count();

    let p2 = iter
        .filter(|[a, b, c, d]| {
            (*a..=*b).contains(c)
                || (*a..=*b).contains(d)
                || (*c..=*d).contains(a)
                || (*c..=*d).contains(b)
        })
        .count();

    (p1, p2)
}

crate::bp!(4);
