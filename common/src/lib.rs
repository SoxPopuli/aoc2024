pub mod grid;
pub mod pipe;
pub mod pos;
pub mod vectors;
pub mod iter;

pub use grid::Grid;
pub use pipe::{Pipe, Tap};
pub use pos::Pos;

use std::{
    io::Read,
    time::{Duration, Instant},
};

pub fn timed<Ret>(func: impl FnOnce() -> Ret) -> (Duration, Ret) {
    let start = Instant::now();
    let res = func();
    let end = Instant::now();

    (end - start, res)
}

fn rolling_mean(items: impl IntoIterator<Item = Duration>) -> Duration {
    let mut average = Duration::from_secs(0);
    let mut iterations = 1;

    for x in items {
        average += (x.saturating_sub(average)) / iterations;
        iterations += 1;
    }

    average
}

pub fn timed_repeated<Ret>(repeats: u32, func: impl Fn() -> Ret) -> (Duration, Ret) {
    let mut res = std::mem::MaybeUninit::uninit();
    let avg = (0..repeats).map(|_| {
        let start = Instant::now();
        res.write(func());
        let end = Instant::now();

        end - start
    });

    (rolling_mean(avg), unsafe { res.assume_init() })
}

pub fn read_stdin() -> String {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn long_func_no_args() {
        std::thread::sleep(Duration::from_secs(1))
    }

    #[test]
    fn timed_no_args() {
        let (time, _) = timed(long_func_no_args);
        assert_eq!(time.as_secs(), Duration::from_secs(1).as_secs())
    }
}
