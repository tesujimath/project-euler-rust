fn main() {
    p1()
}

fn p1() {
    let mut sum = 0;

    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    println!("p1: sum is {}", sum);
}
