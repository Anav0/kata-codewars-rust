use std::collections::VecDeque;

fn dbl_linear(n: u32) -> u32 {
    let mut numbers: Vec<u32> = vec![1];
    let mut queque: VecDeque<u32> = VecDeque::new();
    queque.push_front(1);

    loop {
        if numbers.len() >= (n + 2) as usize {
            break;
        }

        let x = queque.pop_back().unwrap();

        let y = 2 * x + 1;
        let z = 3 * x + 1;

        println!("{} {} {}", x, y, z);

        if !numbers.contains(&y) {
            numbers.push(y);
            queque.push_front(y);
        }

        if !numbers.contains(&z) {
            numbers.push(z);
            queque.push_front(z);
        }
    }
    numbers.sort();
    println!("{:?}", numbers);
    numbers[n as usize]
}

#[test]
fn it_works() {
    assert_eq!(22, dbl_linear(10));
}
