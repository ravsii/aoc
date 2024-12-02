use std::{cmp::max, cmp::min, fs};

fn main() {
    let f = fs::read_to_string("input.txt").expect("file");
    let (mut v1, mut v2): (Vec<_>, Vec<_>) = f
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|v| {
            (
                v.get(0).expect("first").to_owned(),
                v.get(1).expect("second").to_owned(),
            )
        })
        .map(|(i, j)| (i.parse::<i32>().unwrap(), j.parse::<i32>().unwrap()))
        .collect::<Vec<(i32, i32)>>()
        .into_iter()
        .unzip();

    v1.sort();
    v2.sort();

    let zipped = v1.iter().zip(v2.iter().collect::<Vec<_>>());

    let mut result = 0;

    for (i, j) in zipped {
        result += max(i, j) - min(i, j);
    }

    println!("{}", result);
}
