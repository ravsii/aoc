use std::fs;

#[derive(PartialEq, Eq)]
enum Direction {
    Decreasing,
    Increasing,
    NotSet,
}

fn main() {
    let f = fs::read_to_string("input.txt").unwrap();
    let reports = f
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut safe_reports = 0;
    for report in reports {
        let mut iter = report.iter();
        let mut prev = iter.next().unwrap();
        let mut dir = Direction::NotSet;

        let mut is_safe = true;
        for next in iter {
            if next - prev == 0 {
                is_safe = false;
                break;
            }

            let current_dir = if next - prev > 0 {
                Direction::Increasing
            } else {
                Direction::Decreasing
            };

            if dir == Direction::NotSet {
                dir = current_dir
            } else if current_dir != dir {
                is_safe = false;
                break;
            }

            let diff = (next - prev).abs();

            if !(1..=3).contains(&diff) {
                is_safe = false;
                break;
            }

            prev = next;
        }

        if is_safe {
            safe_reports += 1;
        }
    }

    println!("{}", safe_reports);
}
