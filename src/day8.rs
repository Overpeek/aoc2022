fn main(input: &str) -> impl crate::Results {
    let height = input.lines().count();
    let input = input.as_bytes();
    let width = input.iter().take_while(|b| **b != b'\n').count();

    // TODO: faster algo

    let mut count = (width * 2 + height * 2).max(4) - 4;
    let mut max = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let cell = input[x + y + y * width];

            let row = input.iter().skip(y * width + y).take(width);
            let col = input.iter().skip(x).step_by(width + 1).take(height);
            let north = col.clone().take(y).rev();
            let south = col.clone().skip(y + 1);
            let west = row.clone().take(x).rev();
            let east = row.clone().skip(x + 1);

            let cmp = |c: &u8| *c < cell;
            let vis = north.clone().all(cmp)
                || south.clone().all(cmp)
                || west.clone().all(cmp)
                || east.clone().all(cmp);

            let scan = |acc: &mut (i32, bool), height: &u8| {
                acc.0 += 1;
                let res = acc.1;
                acc.1 = *height < cell;
                res.then_some(())
            };
            let score = north.scan((0, true), scan).count()
                * south.scan((0, true), scan).count()
                * west.scan((0, true), scan).count()
                * east.scan((0, true), scan).count();

            count += vis as usize;
            max = max.max(score);
        }
    }

    (count, max)
}

crate::bp!(8);
