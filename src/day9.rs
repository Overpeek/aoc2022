//use std::collections::HashSet;
use rustc_hash::FxHashSet as HashSet;

fn main(input: &str) -> impl crate::Results {
    let (mut head, mut tail) = ((0, 0), (0, 0));
    //let mut visited = HashSet::from([tail]);
    let mut visited = HashSet::default();
    visited.reserve(7_000);
    visited.insert(tail);

    input
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(dir, count)| (dir, count.parse::<i32>().unwrap()))
        .map(|(dir, count)| match dir {
            "R" => (true, count),
            "L" => (true, -count),
            "U" => (false, count),
            "D" => (false, -count),
            _ => unreachable!(),
        })
        .for_each(|(dir, count)| {
            // move head
            *(if dir { &mut head.0 } else { &mut head.1 }) += count;

            // tail weird case
            let dist = (head.0.abs_diff(tail.0), head.1.abs_diff(tail.1));
            if dist.0 == 1 && dist.1 > 1 {
                tail.0 = head.0;
            } else if dist.0 > 1 && dist.1 == 1 {
                tail.1 = head.1;
            }

            // tail normal case
            let dist = (head.0 - tail.0, head.1 - tail.1);
            let mov = (dist.0 - dist.0.signum(), dist.1 - dist.1.signum());
            tail.0 += mov.0;
            tail.1 += mov.1;

            // mark visited
            for i in 0..mov.0.abs() + mov.1.abs() {
                let mut tmp = tail;
                *(if dir { &mut tmp.0 } else { &mut tmp.1 }) -= count.signum() * i;
                visited.insert(tmp);
            }
        });

    (visited.len(), "todo")
}

// fn debug(head: (i32, i32), tail: (i32, i32), visited: &HashSet<(i32, i32)>) {
//     for y in (-8..=8).rev() {
//         for x in -8..=8 {
//             if x == 0 && y == 0 {
//                 print!("H");
//             } else if x == tail.0 - head.0 && y == tail.1 - head.1 {
//                 print!("T");
//             } else if visited.contains(&(x + head.0, y + head.1)) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
//     println!();
// }

crate::bp!(9);
