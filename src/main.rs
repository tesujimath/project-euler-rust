use std::env;
use std::str::FromStr;

fn main() {
    let problems = [p1::run, p2::run, p3::run];

    for arg in env::args().skip(1) {
        let i = usize::from_str(&arg).expect("expected problem number");

        println!("problem {}", i);
        problems[i - 1]();
    }
}

mod p1;
mod p2;
mod p3;
