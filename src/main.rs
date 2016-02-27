use std::ops::Range;

#[test]
fn one_to_thirtythree() {
    let one_to_thirtythree = "1, 2, Fizz Fizz (3), 4, Buzz Buzz (5), Fizz (6), Woof Woof (7), 8, \
                              Fizz (9), Buzz (10), 11, Fizz (12), Fizz (13), Woof (14), Fizz Buzz \
                              Buzz (15), 16, Woof (17), Fizz (18), 19, Buzz (20), Fizz Woof (21), \
                              22, Fizz (23), Fizz (24), Buzz Buzz (25), 26, Fizz Woof (27), Woof \
                              (28), 29, Fizz Fizz Buzz (30), Fizz (31), Fizz (32), Fizz Fizz Fizz \
                              (33)";
    assert_eq!(get_fizz_buzz_woof_string(1..34), one_to_thirtythree);
}

fn matches(i: usize, i_as_string: &String, n: usize, word: &str) -> Vec<String> {
    let mut res = Vec::<String>::new();
    let nc = ('0' as u8 + n as u8) as char;
    if i % n == 0 {
        res.push(String::from(word));
    }
    res.append(&mut i_as_string.chars()
                               .filter(|c| *c == nc)
                               .map(|_| String::from(word))
                               .collect::<Vec<String>>());
    res
}

fn fizz_buzz_woof(i: usize) -> String {
    let mut res = Vec::<String>::new();
    let as_string = i.to_string();
    res.append(&mut matches(i, &as_string, 3, "Fizz"));
    res.append(&mut matches(i, &as_string, 5, "Buzz"));
    res.append(&mut matches(i, &as_string, 7, "Woof"));
    if res.is_empty() {
        res.push(i.to_string());
    } else {
        res.push(format!("({})", i));
    }
    res.join(" ")
}

fn get_fizz_buzz_woof_string(r: Range<usize>) -> String {
    r.map(|i| fizz_buzz_woof(i)).collect::<Vec<String>>().join(", ")
}

fn main() {
    println!("{}", get_fizz_buzz_woof_string(0..100));
}
