// By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. But this takes a lot of work
// The alternative is to immediately abort, which ends the program without cleaning up: `panic = 'abort'` in `[profile]` sections in Cargo.toml
// [profile.release]
// panic = 'abort'

fn main() {
    panic!("crash and burn");

    // Using a panic! Backtrace: `RUST_BACKTRACE=1 cargo run`
    let v = vec![1, 2, 3];

    v[99];
}
