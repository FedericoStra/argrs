use std::io::{stdout, Write};

fn main() {
    let args = std::env::args();
    let argc = args.len();

    // compute the width to print numbers in the range `0..argc`
    let width: usize = if argc <= 10 {
        1
    } else if argc <= 100 {
        2
    } else if argc <= 1_000 {
        3
    } else if argc <= 10_000 {
        4
    } else if argc <= 100_000 {
        5
    } else if argc <= 1_000_000 {
        6
    } else {
        0
    };

    let mut stdout_lock = stdout().lock();

    for (i, arg) in args.enumerate() {
        writeln!(stdout_lock, "{:width$}: `{}`", i, arg).unwrap();
    }
}
