use std::collections::HashMap;

use aoc2017::{run_day, DIAG_DIRS};

fn p1(input: &str) -> i32 {
    let num = input.parse::<i32>().unwrap();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 0;
    let mut min_y = 0;

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut dx = 1;
    let mut dy = 0;

    for _ in 2..=num {
        x += dx;
        y += dy;
        if x > max_x {
            max_x = x;
            dx = 0;
            dy = 1;
        } else if x < min_x {
            min_x = x;
            dx = 0;
            dy = -1;
        } else if y > max_y {
            max_y = y;
            dx = -1;
            dy = 0;
        } else if y < min_y {
            min_y = y;
            dx = 1;
            dy = 0;
        }
    }

    x.abs() + y.abs()
}

fn p2(input: &str) -> i32 {
    let num = input.parse::<i32>().unwrap();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 0;
    let mut min_y = 0;

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut dx = 1;
    let mut dy = 0;

    let mut grid = HashMap::new();
    grid.insert((0, 0), 1);

    loop {
        x += dx;
        y += dy;
        let sum = DIAG_DIRS
            .iter()
            .map(|(dx, dy)| grid.get(&(x + dx, y + dy)).unwrap_or(&0))
            .sum::<i32>();
        grid.insert((x, y), sum);
        if sum > num {
            return sum;
        }
        if x > max_x {
            max_x = x;
            dx = 0;
            dy = 1;
        } else if x < min_x {
            min_x = x;
            dx = 0;
            dy = -1;
        } else if y > max_y {
            max_y = y;
            dx = -1;
            dy = 0;
        } else if y < min_y {
            min_y = y;
            dx = 1;
            dy = 0;
        }
    }
}

fn main() {
    run_day(p1, p2);
}
