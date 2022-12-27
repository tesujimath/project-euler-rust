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
    assert_eq!(is_palindrome(1), true);
    assert_eq!(is_palindrome(12), false);
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(1221), true);
}

pub fn run() {
    let mut largest = 0;
    for i in 100..999 {
        for j in i..999 {
            let n = i * j;
            if is_palindrome(n) && n > largest {
                largest = n;
            }
        }
    }
    println!("p4: {}", largest);
}
