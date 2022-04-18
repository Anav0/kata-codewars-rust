fn rgb(r: i32, g: i32, b: i32) -> String {
    let mut output = String::from("");
    let numbers = [r, g, b];

    for x in numbers {
        let hex = format!("{:X}", x);
        if hex.len() < 2 {
            output.push('0');
        }
        if x < 0 {
            output.push_str("00");
        } else if x > 255 {
            output.push_str("FF");
        } else {
            output.push_str(&hex);
        }
    }

    output
}

#[test]
fn it_works() {
    assert_eq!(rgb(-20, 275, 125), "00FF7D");
    assert_eq!(rgb(0, 0, 0), "000000");
    assert_eq!(rgb(1, 2, 3), "010203");
    assert_eq!(rgb(255, 255, 255), "FFFFFF");
    assert_eq!(rgb(254, 253, 252), "FEFDFC");
    assert_eq!(rgb(0, 250, 0), "00FA00");
}
