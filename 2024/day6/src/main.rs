use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (mut cur_x, mut cur_y) = (0i32, 0i32);
    'pos: for (y, _) in input.iter().enumerate() {
        for (x, _) in input[y].iter().enumerate() {
            if input[y][x] == '^' {
                (cur_x, cur_y) = (x as i32, y as i32);
                break 'pos;
            }
        }
    }

    let mut p1 = HashSet::<(i32, i32)>::new();
    let mut step_x = 0i32;
    let mut step_y = -1i32;

    let border = input.len() as i32;
    loop {
        p1.insert((cur_y, cur_x));

        let (new_x, new_y) = (cur_x + step_x, cur_y + step_y);
        if new_x < 0 || new_x >= border || new_y < 0 || new_y >= border {
            break; // free
        }

        if input[new_y as usize][new_x as usize] == '#' {
            if step_x == 1 {
                step_x = 0;
                step_y = 1;
            } else if step_y == 1 {
                step_x = -1;
                step_y = 0;
            } else if step_x == -1 {
                step_x = 0;
                step_y = -1;
            } else if step_y == -1 {
                step_x = 1;
                step_y = 0;
            }
            continue;
        }

        (cur_x, cur_y) = (new_x, new_y)
    }

    println!("p1 {}", p1.len());
    // println!("p2 {}", p2);
}
