use std::env;
use std::str::FromStr;

mod p1;

fn main() {
    let problems = [p1::run];

    for arg in env::args().skip(1) {
        let i = usize::from_str(&arg).expect("expected problem number");

        println!("problem {}", i);
        problems[i - 1]();
    }
}
