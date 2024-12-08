use std::fs;
use regex::{Captures, Regex};

struct Helper<'a> {
    idx: usize,
    dont: Option<bool>,
    expression: Option<Captures<'a>>
}

fn main() {
    let filename = "./day-3-part-1.txt";
    // let filename = "./smol.txt";
    let line_string = fs::read_to_string(filename).unwrap();
    let line = line_string.as_str();

    part_one(line);
    part_two(line);
}

fn part_one(line: &str) {
    let re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();
    let mut sum = 0;
    re.captures_iter(line).for_each(|c| {
        let a: i32 = c[1].to_string().parse().unwrap();
        let b: i32 = c[2].to_string().parse().unwrap();
        sum += (a*b);
    });
    println!("{}", sum);
}

fn part_two(line: &str) {
    let re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();

    let doo = Regex::new(r"do\(\)").unwrap(); // todo
    let dont = Regex::new(r"don't\(\)").unwrap(); // todo

    let mut helper: Vec<Helper> = Vec::new();
    doo.captures_iter(line).enumerate().for_each(|(i, c)|{
        helper.push(Helper {
            idx: c.get(0).unwrap().start(),
            dont: Some(false),
            expression: None,
        })
    });


    dont.captures_iter(line).enumerate().for_each(|(i, c)| {

        helper.push(Helper {
            idx: c.get(0).unwrap().start(),
            dont: Some(true),
            expression: None,
        })
    });

    helper.sort_by(|a, b| a.idx.cmp(&b.idx));
    re.captures_iter(line).enumerate().for_each(|(i, c)| {
        helper.push(Helper{
            idx: c.get(0).unwrap().start(),
            dont: None,
            expression: Some(c),
        });
    });

    helper.sort_by(|a, b| a.idx.cmp(&b.idx));

    // helper.iter().for_each(|h| {
    //     if h.dont.is_some() {
    //         println!("{}, {}", h.idx, h.dont.unwrap())
    //     }
    // });

    let mut sum = 0;
    let mut evaluate = true;
    for item in helper {
        match item.expression {
            Some(c) => {
                let a: i32 = c[1].to_string().parse().unwrap();
                let b: i32 = c[2].to_string().parse().unwrap();
                if !evaluate {
                    println!("skipping {} * {}", a, b);
                    continue;
                }

                println!("evaluating {} * {}", a, b);
                sum += (a*b);
            }
            None => {
                if item.dont.unwrap() {
                    evaluate = false;
                } else {
                    evaluate = true;
                }
            }
        }
    }
    println!("part 2 -> {}", sum);
}
