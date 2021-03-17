fn stati(strg: &str) -> String {
    if strg.len() <= 0 {
        return String::from("");
    }
    let mut data: Vec<i32> = Vec::new();

    let mut avg: i32 = 0;
    let mut median: i32 = 0;

    for x in strg.split(", ") {
        let info: Vec<&str> = x.split("|").collect();
        let h: i32 = info[0].parse().unwrap();
        let m: i32 = info[1].parse().unwrap();
        let s: i32 = info[2].parse().unwrap();
        let sum = (h * 60 * 60) + (m * 60) + s;
        data.push(sum);
        avg += sum;
    }

    data.sort();

    let mid: usize = data.len() / 2;
    if data.len() % 2 == 0 {
        median = (data[mid] + data[mid - 1]) / 2;
    } else {
        median = data[mid];
    }
    avg /= data.len() as i32;

    format!(
        "Range: {} Average: {} Median: {}",
        s_to_str(data[data.len() - 1] - data[0]),
        s_to_str(avg),
        s_to_str(median)
    )
}

fn s_to_str(mut s: i32) -> String {
    let h = s / (60 * 60);
    s -= h * (60 * 60);
    let m = s / 60;
    s -= m * 60;
    format!("{:0>2}|{:0>2}|{:0>2}", h, m, s)
}

#[test]
fn it_works() {
    assert_eq!(
        "Range: 00|47|18 Average: 01|35|15 Median: 01|32|34",
        stati("01|15|59, 1|47|6, 01|17|20, 1|32|34, 2|3|17"),
    );
    assert_eq!("", stati(""),);
}

#[test]
fn formater_works() {
    assert_eq!("01|00|00", s_to_str(3600));
    assert_eq!("02|00|00", s_to_str(7200));
    assert_eq!("01|20|00", s_to_str(4800));
    assert_eq!("01|22|12", s_to_str(4932));
}
