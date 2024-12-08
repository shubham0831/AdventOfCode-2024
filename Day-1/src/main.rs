use std::fmt::Debug;
use std::{env, fs};
use std::collections::HashMap;
use std::io::Read;

// same test file for both part 1 and part 2
// https://adventofcode.com/2024/day/1
fn main() {
    let filename = "./part-1-test.txt";
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .for_each(|l| {
            let mut iter = l.split_whitespace();
            let l: i32 = iter
                .next()
                .unwrap_or_else(|| panic!("could not find left side"))
                .to_owned()
                .parse()
                .unwrap_or_else(|e| panic!("error in casting string to int {}", e));

            let r: i32 = iter
                .next()
                .unwrap_or_else(|| panic!("could not find right side"))
                .to_owned()
                .parse()
                .unwrap_or_else(|e| panic!("error in casting string to int {}", e));

            left.push(l);
            right.push(r);
        });

    left.sort();
    right.sort();

    let mut sum: i32 = 0;
    for i in 0..left.len() {
        sum += (left.get(i).unwrap() - right.get(i).unwrap()).abs();
    }

    println!("{}", sum);

    let mut right_count = HashMap::new();
    right
        .iter()
        .for_each(|el| *right_count.entry(el).or_insert(0) += 1);

    let mut diff = 0;
    left.iter()
        .for_each(|el| {
            match right_count.get(el) {
                Some(e) => {
                    diff += (el * e);
                }
                None => {}
            }
        });

    println!("diff {}", diff);
}
