use std::{fs, num, slice};

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

    let mut safe_reports_1 = 0;
    for report in &reports {
        if is_safe(report) {
            safe_reports_1 += 1;
        }
    }

    println!("{}", safe_reports_1);

    let mut safe_reports_2 = 0;
    for report in &reports {
        if is_safe(report) {
            safe_reports_2 += 1;
            continue;
        }

        let mut ok = false;
        for (i, _) in report.iter().enumerate() {
            let mut vec = report.clone();
            vec.remove(i);
            if is_safe(&vec) {
                ok = true;
                break;
            }
        }

        if ok {
            safe_reports_2 += 1;
        }
    }

    println!("{}", safe_reports_2);
}

fn is_safe(list: &[i32]) -> bool {
    // either incremental or decremental order
    let order_valid = list.windows(2).all(|w| w[0] < w[1]) || list.windows(2).all(|w| w[0] > w[1]);

    let ok = list.windows(2).all(|w| {
        let diff = i32::abs(w[0] - w[1]);
        (1..=3).contains(&diff)
    });

    order_valid && ok
}
