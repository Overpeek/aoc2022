fn main(input: &str) -> impl crate::Results {
    let mut iter = input.split("$ ").skip(1);

    let mut p1 = 0;
    let total = rec(&mut iter.clone(), &mut |size| {
        if size <= 100_000 {
            p1 += size
        }
    });

    let mut p2 = usize::MAX;
    rec(&mut iter, &mut |size| {
        if size + 40_000_000 >= total && size < p2 {
            p2 = size;
        }
    });

    (p1, p2)
}

fn rec<'a, F: FnMut(usize)>(iter: &mut impl Iterator<Item = &'a str>, f: &mut F) -> usize {
    let mut size = 0_usize;

    while let Some(cmd) = iter.next() {
        if cmd.starts_with("cd ..") {
            break;
        } else if cmd.starts_with("c") {
            size += rec(iter, f)
        } else {
            let res = &cmd[3..];
            size += res
                .lines()
                .filter_map(|s| s.split_once(' '))
                .filter_map(|(size, _)| size.parse::<usize>().ok())
                .sum::<usize>();
        };
    }

    f(size);

    size
}

crate::bp!(7);
