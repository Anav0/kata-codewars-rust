//https://www.codewars.com/kata/5502c9e7b3216ec63c0001aa/train/rust
//Categorize new member

fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    let mut output = Vec::with_capacity(data.len());
    for (age, handicap_level) in data {
        if age >= 55 && handicap_level > 7 {
            output.push(String::from("Senior"));
        } else {
            output.push(String::from("Open"));
        }
    }
    output
}

#[test]
fn it_works() {
    let data = vec![(18, 20), (45, 2), (61, 12), (37, 6), (21, 21), (78, 9)];
    let output = open_or_senior(data);
    assert_eq!(
        vec!["Open", "Open", "Senior", "Open", "Open", "Senior"],
        output
    );
}
