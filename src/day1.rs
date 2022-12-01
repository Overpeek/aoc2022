fn main(input: &str) -> (i32, i32) {
    let mut top_3 = [0, 0, 0];
    for cals in input.split("\n\n").map(|elf| {
        elf.lines()
            .map(|s| s.parse::<i32>().expect("Input format error"))
            .sum::<i32>()
    }) {
        let v = top_3
            .iter_mut()
            .min()
            .expect("`top_3` is certainly not empty");
        *v = cals.max(*v);
    }

    (
        top_3.iter().copied().max().unwrap_or(0),
        top_3.iter().sum::<i32>(),
    )
}

crate::bp!(1);
