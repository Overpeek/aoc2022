fn main(input: &str) -> impl crate::Results {
    let iter = input.lines().map(|s| {
        s.split(|c| c == ',' || c == '-')
            .map(|s| s.parse::<u32>().unwrap())
            .array_chunks::<4>()
            .next()
            .unwrap()
    });

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
