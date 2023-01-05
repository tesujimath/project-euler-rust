#[macro_use]
extern crate proptest;

use clap::Parser;
use num::BigUint;
use std::process::ExitCode;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    problem: Option<usize>,

    #[arg(long)]
    sieve_u64: Option<usize>,

    #[arg(long)]
    sieve_u64g: Option<usize>,

    #[arg(long)]
    sieve_u128g: Option<usize>,

    #[arg(long)]
    sieve_u128gr: Option<usize>,

    #[arg(long)]
    sieve_b: Option<usize>,

    #[arg(long)]
    sieve_bg: Option<usize>,

    #[arg(long)]
    primes_u64: Option<usize>,

    #[arg(long)]
    primes_u64g: Option<usize>,
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
    } else if let Some(n) = cli.sieve_u64 {
        let mut sieve = primes::FixedSieve::new();
        let p = sieve.get(n);
        println!("{}th prime from u64 is {:?}", n, p);
    } else if let Some(n) = cli.sieve_u64g {
        let mut sieve: primes::Sieve<u64> = primes::Sieve::new();
        let p = sieve.get(n);
        println!("{}th prime from u64g is {:?}", n, p);
    } else if let Some(n) = cli.sieve_u128g {
        let mut sieve: primes::Sieve<u128> = primes::Sieve::new();
        let p = sieve.get(n);
        println!("{}th prime from u128g is {:?}", n, p);
    } else if let Some(n) = cli.sieve_u128gr {
        let mut sieve: primes::RefSieve<u128> = primes::RefSieve::new();
        let p = sieve.get(n);
        println!("{}th prime from u128g is {:?}", n, p);
    } else if let Some(n) = cli.sieve_b {
        let mut sieve = primes::BigSieve::new();
        let p = sieve.get(n);
        println!("{}th prime from b is {:?}", n, p);
    } else if let Some(n) = cli.sieve_bg {
        let mut sieve: primes::RefSieve<BigUint> = primes::RefSieve::new();
        let p = sieve.get(n);
        println!("{}th prime from bg is {:?}", n, p);
    } else if let Some(n) = cli.primes_u64 {
        let mut sieve = primes::FixedSieve::new();
        for i in 0..n {
            println!("{}", sieve.get(i));
        }
    } else if let Some(n) = cli.primes_u64g {
        let mut sieve: primes::Sieve<u64> = primes::Sieve::new();
        for i in 0..n {
            println!("{:?}", sieve.get(i));
        }
    } else {
        println!("default run target");
        let mut sieve: primes::RefSieve<u64> = primes::RefSieve::new();
        for i in 0..10 {
            println!("{:?}", sieve.get(i));
        }
        let p = sieve.get(100);
        println!("{:?}", p);
        //let mut s0 = sieve.skip(5_usize);
        //let mut s1 = s0.skip(1_usize);
        //let mut s2 = s1.skip(1_usize);
        //let p = s2.next();
        //println!("final is {:?}", p);
    }

    ExitCode::SUCCESS
}

mod p1;
mod p2;
mod p3;
mod p4;

mod primes;
