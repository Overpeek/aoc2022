fn main(input: &str) -> impl crate::Results {
    let mut top_3 = [0, 0, 0];
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|s| s.parse::<i32>().unwrap()).sum::<i32>())
        .for_each(|cals| {
            let v = top_3.iter_mut().min().unwrap();
            *v = cals.max(*v);
        });

    (
        top_3.iter().copied().max().unwrap_or(0),
        top_3.iter().sum::<i32>(),
    )
}

crate::bp!(1);
