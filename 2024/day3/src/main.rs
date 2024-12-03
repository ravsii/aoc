use std::fs;

use regex::Regex;

const REGEX_STR: &str = r"mul\((\d{1,3}),(\d{1,3})\)";
const REGEX_STR2: &str = r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\)|don't\(\))";

fn main() {
    let f = fs::read_to_string("input.txt").unwrap();

    let p1 = Regex::new(REGEX_STR)
        .unwrap()
        .captures_iter(f.as_str())
        .map(|capture| {
            capture.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * capture.get(2).unwrap().as_str().parse::<i32>().unwrap()
        })
        .sum::<i32>();

    println!("p1 {}", p1);

    let mut do_ = true;
    let p2 = Regex::new(REGEX_STR2)
        .unwrap()
        .captures_iter(f.as_str())
        .map(|capture| {
            let get = capture.get(0);
            let get = get.unwrap().as_str();

            if get == "don't()" {
                do_ = false;
                0
            } else if get == "do()" {
                do_ = true;
                0
            } else if do_ {
                capture.get(1).unwrap().as_str().parse::<i32>().unwrap()
                    * capture.get(2).unwrap().as_str().parse::<i32>().unwrap()
            } else {
                0
            }
        })
        .sum::<i32>();

    println!("p2: {}", p2);
}
