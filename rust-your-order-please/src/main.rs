#![allow(warnings)]

use std::vec;
fn main() {
    let sentence = "is2 Thi1s T4est 3a";
    println!("{}",order(&sentence));

}

fn order(sentence: &str) -> String {
    if sentence.is_empty() {
        return sentence.to_string();
    } 
    let words: Vec<&str> = sentence.split(' ').collect();
    let mut co_sentence: Vec<String> = vec![String::new();words.len()];
    for word in words.iter() {
        if let Some(digit) = word.chars().find(|c| c.is_digit(10)) {
            let index: usize = digit.to_string().parse().unwrap();
            co_sentence[index-1] = word.to_string();
        } else {
            dbg!("no digit in word");
        }
    }
    co_sentence.join(" ")
}

fn order_v2(sentence: &str) -> String {
    let mut ws: Vec<_> = sentence.split_whitespace().collect();
    ws.sort_by_key(|s| s.chars().find(|c| c.is_digit(10)).unwrap());
    ws.join(" ")
}


