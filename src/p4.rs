use std::string::ToString;

fn is_palindrome<T>(n: T) -> bool
where
    T: ToString,
{
    let s1 = n.to_string().chars().collect::<Vec<char>>();
    let s2 = s1.iter().rev();
    s1.iter().zip(s2).all(|(c1, c2)| c1 == c2)
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome(1));
    assert!(!is_palindrome(12));
    assert!(is_palindrome(121));
    assert!(is_palindrome(1221));
}

fn try_all() -> i32 {
    let mut iterations = 0;
    let mut largest = 0;
    for i in 100..999 {
        for j in i..999 {
            let n = i * j;
            iterations += 1;
            if is_palindrome(n) && n > largest {
                largest = n;
            }
        }
    }

    println!("p4::try_all {} iterations", iterations);

    largest
}

fn opportunistic() -> i32 {
    let mut iterations = 0;
    let mut largest = 0;
    const UPPER: i32 = 999;
    const LOWER: i32 = 100;
    let mut i = UPPER;
    while i * UPPER > largest && i >= LOWER {
        let mut j = UPPER;

        while i * j > largest && j >= LOWER {
            let n = i * j;
            iterations += 1;
            if is_palindrome(n) && n > largest {
                largest = n;
            }

            j -= 1;
        }

        i -= 1;
    }

    println!("p4::opportunistic {} iterations", iterations);

    largest
}

pub fn run() {
    println!("p4: {} {}", try_all(), opportunistic());
}
