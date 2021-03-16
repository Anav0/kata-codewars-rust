pub fn digital_root(n: i64) -> i64 {
    let digits: Vec<i64> = n
        .to_string()
        .chars()
        .map(|c| i64::from(c.to_digit(10).unwrap())) //Is very slow
        .collect();

    if digits.len() == 1 {
        return n;
    }

    let sum: i64 = digits.iter().sum();
    println!("{:?} {}", digits, sum);
    digital_root(sum)
}

#[test]
fn it_works() {
    let test_set = vec![(12, 3), (786, 3)];

    for set in test_set {
        assert_eq!(digital_root(set.0), set.1);
    }
}
