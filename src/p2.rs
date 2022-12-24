use std::ops::Add;

struct Fib<T>(T, T);

impl<T> Fib<T>
where
    T: Add<Output = T> + Copy,
{
    pub fn next(&mut self) -> T {
        let result = self.0;
        let sum = self.0 + self.1;
        self.0 = self.1;
        self.1 = sum;
        result
    }
}

pub fn run() {
    let mut fib = Fib(1, 2);

    for _ in 1..10 {
        println!("{}", fib.next());
    }
}
