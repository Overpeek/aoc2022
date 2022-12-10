fn main(input: &str) -> impl crate::Results {
    let width = input.as_bytes().iter().take_while(|c| **c != b'\n').count();

    let mut grid: Vec<_> = input
        .as_bytes()
        .iter()
        .filter(|c| **c != b'\n')
        .map(|c| (false, 1, *c))
        .collect();

    for row in grid.chunks_mut(width) {
        ray_cast(row.iter_mut());
        ray_cast(row.iter_mut().rev());
    }
    for x in 0..width {
        ray_cast(grid.iter_mut().skip(x).step_by(width));
        ray_cast(grid.iter_mut().skip(x).step_by(width).rev());
    }

    // println!("{input}");
    // for line in grid.chunks(width) {
    //     for (v, _, _) in line {
    //         if *v {
    //             print!("y");
    //         } else {
    //             print!("n");
    //         }
    //     }
    //     println!();
    // }

    let p1 = grid.iter().filter(|(vis, _, _)| *vis).count();
    let p2 = grid.iter().map(|(_, score, _)| *score).max().unwrap_or(0);

    (p1, p2)
}

fn ray_cast(iter: impl Iterator<Item = &mut (bool, usize, u8)>) {
    // p1
    let mut highest = 0;
    // p2
    let mut index_of_last_specific_height = [usize::MAX; 11];
    index_of_last_specific_height[10] = 0;
    // shared
    for (i, tree) in iter.enumerate() {
        // p1
        if highest < tree.2 {
            highest = tree.2;
            tree.0 = true;
        }
        // p2
        let arr_i = (tree.2 - b'0') as usize;
        tree.1 *= index_of_last_specific_height
            .iter()
            .skip(arr_i)
            .filter(|s| **s != usize::MAX)
            .map(|index| i - *index)
            .min()
            .unwrap_or(0);
        index_of_last_specific_height[arr_i] = i;
    }
}

crate::bp!(8);
