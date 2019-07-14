fn main() {
    let args = std::env::args();
    let argc = args.len();
    let width: usize;
    if argc <= 10 {
        width = 1
    } else if argc <= 100 {
        width = 2
    } else if argc <= 1_000 {
        width = 3
    } else if argc <= 10_000 {
        width = 4
    } else if argc <= 100_000 {
        width = 5
    } else if argc <= 1_000_000 {
        width = 6
    } else {
        width = 0
    }
    for (i, arg) in args.enumerate() {
        println!("{:width$}: `{}`", i, arg, width = width);
    }
}
