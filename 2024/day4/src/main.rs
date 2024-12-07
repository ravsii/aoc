use std::fs;

enum Direction {
    Up,
    UpRight,
    Right,
    RightDown,
    Down,
    LeftDown,
    Left,
    UpLeft,
}

impl Direction {
    fn next(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::UpRight => (-1, 1),
            Direction::Right => (0, 1),
            Direction::RightDown => (1, 1),
            Direction::Down => (1, 0),
            Direction::LeftDown => (1, -1),
            Direction::Left => (0, -1),
            Direction::UpLeft => (-1, -1),
        }
    }
}

fn main() {
    let lines = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().to_owned().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut p1 = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let current = lines[i][j];
            if current != 'X' {
                continue;
            }

            p1 += go1(i as i32, j as i32, Direction::Up, 'X', &lines);
            p1 += go1(i as i32, j as i32, Direction::UpRight, 'X', &lines);
            p1 += go1(i as i32, j as i32, Direction::Right, 'X', &lines);
            p1 += go1(i as i32, j as i32, Direction::RightDown, 'X', &lines);
            p1 += go1(i as i32, j as i32, Direction::Down, 'X', &lines);
            p1 += go1(i as i32, j as i32, Direction::LeftDown, 'X', &lines);
            p1 += go1(i as i32, j as i32, Direction::Left, 'X', &lines);
            p1 += go1(i as i32, j as i32, Direction::UpLeft, 'X', &lines);
        }
    }

    println!("p1 {}", p1);

    let mut p2 = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if go2(i as i32, j as i32, &lines) {
                p2 += 1;
            }
        }
    }

    println!("p2 {}", p2);
}

fn go1(i: i32, j: i32, dir: Direction, look_for: char, lines: &Vec<Vec<char>>) -> i32 {
    let max = lines.len() as i32;
    if i < 0 || i >= max || j < 0 || j >= max {
        return 0;
    }

    let current = lines[i as usize][j as usize];

    if current != look_for {
        return 0;
    }

    if current == 'S' && look_for == 'S' {
        return 1;
    }

    let next = match current {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => return 0,
    };

    let (ni, nj) = dir.next();

    go1(i + ni, j + nj, dir, next, lines)
}

fn go2(i: i32, j: i32, lines: &[Vec<char>]) -> bool {
    let max_i = (lines.len() - 2) as i32;
    if i < 0 || i >= max_i {
        return false;
    }

    let max_j = (lines[i as usize].len() - 2) as i32;
    if j < 0 || j >= max_j {
        return false;
    }

    let (i, j) = (i as usize, j as usize);

    if lines[i + 1][j + 1] != 'A' {
        return false;
    };

    if lines[i][j] == 'M'
        && lines[i][j + 2] == 'M'
        && lines[i + 2][j] == 'S'
        && lines[i + 2][j + 2] == 'S'
    {
        return true;
    }

    if lines[i][j] == 'M'
        && lines[i][j + 2] == 'S'
        && lines[i + 2][j] == 'M'
        && lines[i + 2][j + 2] == 'S'
    {
        return true;
    }

    if lines[i][j] == 'S'
        && lines[i][j + 2] == 'S'
        && lines[i + 2][j] == 'M'
        && lines[i + 2][j + 2] == 'M'
    {
        return true;
    }

    if lines[i][j] == 'S'
        && lines[i][j + 2] == 'M'
        && lines[i + 2][j] == 'S'
        && lines[i + 2][j + 2] == 'M'
    {
        return true;
    }

    false
}
