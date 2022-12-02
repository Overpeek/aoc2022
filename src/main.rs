use std::{
    env::args,
    fmt::Display,
    sync::atomic::{AtomicBool, Ordering},
};

//

mod day1;
mod day2;

//

static TEST_MODE: AtomicBool = AtomicBool::new(false);

//

fn main() {
    if args().any(|s| s == "-t") {
        TEST_MODE.store(true, Ordering::SeqCst);
    }

    day1::run();
    day2::run();
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

            crate::Results::print(&main(input), $n)
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
    fn print(&self, day: u8);
}

impl<T: Display, U: Display> Results for (T, U) {
    fn print(&self, day: u8) {
        println!(
            "Day {day} results:\nPart 1: {}\nPart 2: {}\n\n",
            self.0, self.1
        );
    }
}
