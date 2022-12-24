use std::ops::Add;

struct Fib<T>(T, T);

impl<T> Iterator for Fib<T>
where
    T: Add<Output = T> + Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let result = self.0;
        let sum = self.0 + self.1;
        self.0 = self.1;
        self.1 = sum;
        Some(result)
    }
}

fn is_even(n: &i32) -> bool {
    n % 2 == 0
}

pub fn run() {
    let fib = Fib(1, 2);
    const MAX: i32 = 4_000_000;

    let sum: i32 = fib
        //.inspect(|n| println!("all {}", n))
        .filter(is_even)
        //.inspect(|n| println!("even {}", n))
        .take_while(|n| n <= &MAX)
        .sum();
    println!("p2: sum is {}", sum);
}
