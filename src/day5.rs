fn main(input: &str) -> impl crate::Results {
    // prep

    let (crates, instr) = input.split_once("\n\n").unwrap();
    let (crates, counts) = crates.rsplit_once('\n').unwrap();

    // parse stacks

    let mut stacks = Vec::<Vec<_>>::new();
    stacks.resize(counts.split_whitespace().count(), Vec::new());
    crates
        .lines()
        .take_while(|l| l.contains('['))
        .map(str::chars)
        .for_each(|s| {
            s.skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| *c != ' ')
                .for_each(|(i, c)| {
                    stacks[i].insert(0, c);
                });
        });

    // parse instructions

    let instr = instr.lines().map(|s| {
        s.split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|s| s.parse::<usize>().unwrap() - 1)
            .array_chunks::<3>()
            .next()
            .unwrap()
    });

    // part 1

    let mut p1 = stacks.clone();
    for [count, from, to] in instr.clone() {
        for _ in 0..count + 1 {
            let cont = p1[from].pop().unwrap();
            p1[to].push(cont);
        }
    }

    let p1 = p1
        .into_iter()
        .filter_map(|mut stack| stack.pop())
        .collect::<String>();

    // part 2

    let mut p2 = stacks;
    for [count, from, to] in instr.clone() {
        let (from, to) = if from > to {
            let (a, b) = p2.split_at_mut(to + 1);
            (&mut b[from - to - 1], a.last_mut().unwrap())
        } else {
            let (a, b) = p2.split_at_mut(from + 1);
            (a.last_mut().unwrap(), &mut b[to - from - 1])
        };

        to.extend(from.drain(from.len() - count - 1..));
    }

    let p2 = p2
        .into_iter()
        .filter_map(|mut stack| stack.pop())
        .collect::<String>();

    (p1, p2)
}

crate::bp!(5);
