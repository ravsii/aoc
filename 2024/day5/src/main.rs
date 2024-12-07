use std::{collections::HashMap, fs};

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n\n").collect::<Vec<_>>();
    let (ordering, updates) = (input[0], input[1]);

    let mut m = HashMap::<i32, Vec<i32>>::new();

    ordering
        .split("\n")
        .map(|line| {
            let (before, after) = line.split_once("|").unwrap();
            (
                before.parse::<i32>().unwrap(),
                after.parse::<i32>().unwrap(),
            )
        })
        .for_each(|(b, a)| {
            match m.get_mut(&b) {
                Some(v) => {
                    v.push(a);
                }
                None => {
                    m.insert(b, vec![a]);
                }
            };
        });

    let mut p1 = 0;
    updates
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(",")
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|line| {
            for (i, n) in line.iter().enumerate() {
                let vec = Vec::new();
                let v = m.get(n).unwrap_or(&vec);
                if v.is_empty() {
                    continue;
                }

                for j in (0..i).rev() {
                    if v.contains(&line[j]) {
                        return None;
                    }
                }
            }

            Some(line)
        })
        .for_each(|line| {
            if let Some(line) = line {
                p1 += line[line.len() / 2]
            }
        });

    println!("p1 {}", p1);

    let mut p2 = 0;
    updates
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(",")
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter_map(|line| {
            for (i, n) in line.iter().enumerate() {
                let vec = Vec::new();
                let v = m.get(n).unwrap_or(&vec);
                if v.is_empty() {
                    continue;
                }

                for j in (0..i).rev() {
                    if v.contains(&line[j]) {
                        return Some(line);
                    }
                }
            }

            None
        })
        .map(|mut line| {
            let mut i = 0;
            while i < line.len() {
                let vec = Vec::new();
                let v = m.get(&line[i]).unwrap_or(&vec);
                if v.is_empty() {
                    i += 1;
                    continue;
                }

                for j in (0..i).rev() {
                    if v.contains(&line[j]) {
                        line.swap(i, j);
                        i -= 1;
                    }
                }

                i += 1;
            }

            line
        })
        .for_each(|line| {
            let var_name = line[line.len() / 2];
            println!("{}", var_name);
            p2 += var_name
        });
    println!("p2 {}", p2);
}
