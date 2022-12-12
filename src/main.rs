#![feature(iter_array_chunks)]
#![feature(split_array)]
#![feature(anonymous_lifetime_in_impl_trait)]

//

use std::{
    env::args,
    fmt::Display,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    time::Duration,
};

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

//

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
// gen mod

//

static TEST_MODE: AtomicBool = AtomicBool::new(false);
static BATCH_COUNT: AtomicUsize = AtomicUsize::new(1);

//

fn main() {
    if args().any(|s| s == "-t") {
        TEST_MODE.store(true, Ordering::SeqCst);
    }
    if let Some(n) = args().find(|s| s.starts_with("-n")) {
        BATCH_COUNT.store(
            n[2..]
                .parse()
                .expect("Failed to parse the batch count number"),
            Ordering::SeqCst,
        );
    }

    let start = std::time::Instant::now();
    let results = [
        day1::run,
        day2::run,
        day3::run,
        day4::run,
        day5::run,
        day6::run,
        day7::run,
        day8::run,
        day9::run,
        // gen run
    ]
    .into_par_iter()
    .map(|s| s())
    .collect::<Vec<_>>();
    for result in results {
        println!("{result}");
    }

    let time = start.elapsed();
    println!("Total time: {time:?}");
}

#[macro_export]
macro_rules! bp {
    ($n:expr) => {
        pub fn run() -> String {
            let input = if crate::TEST_MODE.load(std::sync::atomic::Ordering::SeqCst) {
                include_str!(concat!(
                    concat!(concat!("input/day"), stringify!($n)),
                    "_test.txt"
                ))
            } else {
                include_str!(concat!(
                    concat!(concat!("input/day"), stringify!($n)),
                    ".txt"
                ))
            };

            let start = std::time::Instant::now();
            let result = (0..crate::BATCH_COUNT.load(std::sync::atomic::Ordering::SeqCst))
                .map(|_| main(input))
                .last()
                .expect("Batch count of 0 is not allowed");
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
    fn print(&self, day: u8, time: Duration) -> String;
}

impl<T: Display, U: Display> Results for (T, U) {
    fn print(&self, day: u8, time: Duration) -> String {
        format!(
            "Day {day} results:\nPart 1: {}\nPart 2: {}\nTime: {time:?}\n",
            self.0, self.1
        )
    }
}
