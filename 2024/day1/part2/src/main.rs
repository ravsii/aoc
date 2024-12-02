use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
};

fn main() {
    let f = fs::read_to_string("input.txt").expect("file");
    let (mut list1, mut list2): (Vec<_>, Vec<_>) = f
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

    let mut m = HashMap::<i32, i32>::new();

    for item in list2 {
        match m.get_mut(&item) {
            Some(val) => {
                *val = val.saturating_add(1);
            }
            None => {
                m.insert(item, 1);
            }
        }
    }

    let mut result = 0;
    for item in list1 {
        result += item * m.get(&item).unwrap_or(&0);
    }

    println!("{}", result);
}
