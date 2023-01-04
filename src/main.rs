#[macro_use]
extern crate proptest;

use clap::Parser;
use lending_iterator::LendingIterator;
use std::env;
use std::iter::Iterator;
use std::process::ExitCode;
use std::str::FromStr;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    problem: Option<usize>,

    #[arg(long)]
    sieve_u64: Option<usize>,

    #[arg(long)]
    sieve_u64g: Option<usize>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let problems = [p1::run, p2::run, p3::run, p4::run];

    if let Some(p) = cli.problem {
        if let Some(f) = problems.get(p - 1) {
            f()
        } else {
            println!("problem index {} out of bounds 1..{}", p, problems.len());

            return ExitCode::FAILURE;
        }
    }

    if let Some(n) = cli.sieve_u64 {
        let mut sieve = primes::Sieve::new();
        let p = sieve.nth(n);
        println!("{}th prime from u64 is {:?}", n, p);
    }

    // TODO my LendingIterator does not skip over any values 🤷
    if let Some(n) = cli.sieve_u64g {
        let mut sieve: genprimes::Sieve<u64> = genprimes::Sieve::new();
        let p = sieve.nth(n);
        println!("{}th prime from u64g is {:?}", n, p);
    }

    ExitCode::SUCCESS
}

mod p1;
mod p2;
mod p3;
mod p4;
mod primes;

mod bigprimes;
mod genprimes;
