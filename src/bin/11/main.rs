use aoc2017::run_day;
use std::cmp::max;

// general idea is use your horizontal steps to also move by 1 vertically (nw,ne,sw,se)
// then take the remaining vertical steps and divide by 2
fn dist(x: i32, y: i32) -> i32 {
    let x_steps = x.abs() / 2;
    let y_steps = max(y.abs() - x_steps, 0) / 2;
    x_steps + y_steps
}

fn p1(input: &str) -> i32 {
    let mut tx: i32 = 0;
    let mut ty: i32 = 0;

    input.split(',').for_each(|dir| match dir {
        "ne" => {
            tx += 2;
            ty += 1;
        }
        "n" => {
            ty += 2;
        }
        "nw" => {
            tx -= 2;
            ty += 1;
        }
        "sw" => {
            tx -= 2;
            ty -= 1;
        }
        "s" => {
            ty -= 2;
        }
        "se" => {
            tx += 2;
            ty -= 1;
        }
        _ => unreachable!("Unknown direction: {}", dir),
    });

    dist(tx, ty)
}

fn p2(input: &str) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut max_steps = 0;

    input.split(',').for_each(|dir| {
        match dir {
            "ne" => {
                x += 2;
                y += 1;
            }
            "n" => {
                y += 2;
            }
            "nw" => {
                x -= 2;
                y += 1;
            }
            "sw" => {
                x -= 2;
                y -= 1;
            }
            "s" => {
                y -= 2;
            }
            "se" => {
                x += 2;
                y -= 1;
            }
            _ => unreachable!("Unknown direction: {}", dir),
        }
        max_steps = max(max_steps, dist(x, y));
    });

    max_steps
}

fn main() {
    run_day(p1, p2);
}
