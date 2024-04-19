use std::collections::HashSet;

use aoc2017::run_day;

fn p1(input: &str) -> i32 {
    let split: Vec<&str> = input.split('\n').collect();
    let mut infected = HashSet::new();
    split.iter().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                infected.insert((x as i32, y as i32));
            }
        });
    });

    let (mut x, mut y) = (split[0].len() as i32 / 2, split.len() as i32 / 2);
    let (mut dx, mut dy) = (0, -1);
    let mut num_infections = 0;
    for _ in 0..10_000 {
        if infected.contains(&(x, y)) {
            infected.remove(&(x, y));
            let (_dx, _dy) = (-dy, dx);
            dx = _dx;
            dy = _dy;
        } else {
            infected.insert((x, y));
            let (_dx, _dy) = (dy, -dx);
            dx = _dx;
            dy = _dy;
            num_infections += 1;
        }
        x += dx;
        y += dy;
    }

    num_infections
}

fn p2(input: &str) -> i32 {
    let split: Vec<&str> = input.split('\n').collect();
    let mut infected = HashSet::new();
    split.iter().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                infected.insert((x as i32, y as i32));
            }
        });
    });

    let mut flagged = HashSet::new();
    let mut weakened = HashSet::new();
    let (mut x, mut y) = (split[0].len() as i32 / 2, split.len() as i32 / 2);
    let (mut dx, mut dy) = (0, -1);
    let mut num_infections = 0;
    for _ in 0..10_000_000 {
        if infected.contains(&(x, y)) {
            infected.remove(&(x, y));
            flagged.insert((x, y));
            let (_dx, _dy) = (-dy, dx);
            dx = _dx;
            dy = _dy;
        } else if weakened.contains(&(x, y)) {
            weakened.remove(&(x, y));
            infected.insert((x, y));
            num_infections += 1;
        } else if flagged.contains(&(x, y)) {
            flagged.remove(&(x, y));
            let (_dx, _dy) = (-dx, -dy);
            dx = _dx;
            dy = _dy;
        } else {
            weakened.insert((x, y));
            let (_dx, _dy) = (dy, -dx);
            dx = _dx;
            dy = _dy;
        }
        x += dx;
        y += dy;
    }

    num_infections
}

fn main() {
    run_day(p1, p2);
}
