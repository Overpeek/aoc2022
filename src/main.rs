use std::{
    env::args,
    fmt::Display,
    sync::atomic::{AtomicBool, Ordering},
};

//

mod day1;

//

static TEST_MODE: AtomicBool = AtomicBool::new(false);

//

fn main() {
    if args().any(|s| s == "-t") {
        TEST_MODE.store(true, Ordering::SeqCst);
    }

    day1::run();
}

#[macro_export]
macro_rules! bp {
    ($n:expr) => {
        pub fn run() {
            let input = if crate::TEST_MODE.load(std::sync::atomic::Ordering::SeqCst) {
                include_str!(concat!(
                    concat!(concat!("day"), stringify!($n)),
                    "_test.txt"
                ))
            } else {
                include_str!(concat!(concat!(concat!("day"), stringify!($n)), ".txt"))
            };

            let (part1, part2) = main(input);
            crate::print_result($n, part1, part2);
        }
    };
}

fn print_result<T: Display, U: Display>(day: u8, part1: T, part2: U) {
    println!("Day {day} results:\nPart 1: {part1}\nPart 2: {part2}\n\n");
}
