fn main() {
    let result = high_and_low("1 2 3 -10 -20 -30");
    dbg!(result);
}

fn high_and_low(numbers: &str) -> String {
    let mut min: i32 = 0;
    let mut max: i32 = 0;
    for (index, str_number) in numbers.split_whitespace().enumerate() {
        let n = str_number.parse().unwrap();
        if index == 0 {
            min = n;
            max = n;
            continue;
        }
        if n > max { max = n;}
        if n < min { min = n;}
    }
    let mut newstr = max.to_string();
    newstr.push(' ');
    newstr.push_str(&min.to_string());
    newstr
}

fn high_and_low_v2(numbers: &str) -> String {
    use std::cmp;
    let f = |(max, min), x| (cmp::max(max, x), cmp::min(min, x));

    let answer = numbers
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .fold((i32::min_value(), i32::max_value()), f);
    format!("{} {}", answer.0, answer.1)
}

fn high_and_low_v3(numbers: &str) -> String {
  let as_ints: Vec<i32> = numbers.split(" ").map(|x| x.parse().unwrap()).collect();
  format!("{} {}", as_ints.iter().max().unwrap(), as_ints.iter().min().unwrap())
}
