use std::char::from_u32;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fs;
use std::process::id;

// https://adventofcode.com/2024/day/2
fn main() {
    let filename = "./part-1-test.txt";
    let vec: Vec<Vec<i32>> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let mut ve: Vec<i32> = Vec::new();
            line.split_whitespace()
                .for_each(|line| {
                    ve.push(line.parse().unwrap());
                });
                ve
        })
        .collect();
    part_one(&vec);
    part_two(&vec);
}

fn is_valid(input: &Vec<i32>) -> bool {
    let mut valid_inc = true;
    let mut valid_dec = true;

    for win in input.windows(2) {
        let prev = win[0];
        let next = win[1];

        let abs_difference = prev.abs_diff(next);

        if abs_difference < 1 || abs_difference > 3 {
            return false;
        }

        if prev > next {
            valid_inc = valid_inc & false;
        } else if next > prev {
            valid_dec = valid_dec & false;
        }
    }
    return valid_inc || valid_dec;
}

fn part_one(input: &Vec<Vec<i32>>) {
    let mut valid_count = 0;
    input
        .iter()
        .for_each(|vec| {
            if (is_valid(vec)) {
                valid_count += 1;
            }
        });

    println!("part 1 - {}", valid_count);
}

// https://adventofcode.com/2024/day/2#part2
fn part_two(input: &Vec<Vec<i32>>) {
    let mut valid_count = 0;

    for vec in input {
        if is_valid(vec) {
            valid_count += 1;
            continue;
        }

        let mut found_valid = false;
        for i in 0..vec.len(){
            let new_vec = exclude_idx(i, vec);
            if is_valid(&new_vec) {
                found_valid = true;
                break;
            }
        }

        if found_valid {
            valid_count += 1;
        }
    }
    println!("part 2 - {}", valid_count);
}

fn exclude_idx (i: usize, vec: &Vec<i32>) -> Vec<i32> {
    vec
        .iter()
        .enumerate()
        .filter(|(j, _)| *j != i)
        .map(|(_, el)| el.clone())
        .collect()
}