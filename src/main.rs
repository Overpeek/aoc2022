#![feature(iter_array_chunks)]
#![feature(split_array)]

//

use std::{
    env::args,
    fmt::Display,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

//

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
// gen mod

//

static TEST_MODE: AtomicBool = AtomicBool::new(false);

//

fn main() {
    if args().any(|s| s == "-t") {
        TEST_MODE.store(true, Ordering::SeqCst);
    }

    let start = std::time::Instant::now();
    day1::run();
    day2::run();
    day3::run();
    day4::run();
    day5::run();
    day6::run();
    // gen run
    let time = start.elapsed();
    println!("Total time: {time:?}");
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

            let start = std::time::Instant::now();
            let result = main(input);
            let time = start.elapsed();

            crate::Results::print(&result, $n, time)
        }
    };
}

#[macro_export]
macro_rules! match_unwrap {
    ($test:expr, $pat:tt => $code:tt) => {
        match $test {
            $pat => $code,
            _ => unreachable!(),
        }
    };
}

trait Results {
    fn print(&self, day: u8, time: Duration);
}

impl<T: Display, U: Display> Results for (T, U) {
    fn print(&self, day: u8, time: Duration) {
        println!(
            "Day {day} results:\nPart 1: {}\nPart 2: {}\nTime: {time:?}\n",
            self.0, self.1
        );
    }
}
