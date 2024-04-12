use aoc2017::run_day;

fn p1(input: &str) -> String {
    let grid: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    let mut x: i32 = grid[0].iter().position(|&c| c == '|').unwrap() as i32;
    let mut y: i32 = 0;
    let mut dx: i32 = 0;
    let mut dy: i32 = 1;

    let mut path_letters = Vec::new();
    loop {
        let c = grid[y as usize][x as usize];
        if c.is_alphabetic() {
            path_letters.push(c);
        }

        let dir_options = [(dx, dy), (-dy, -dx), (dy, dx)];
        let next_dir = dir_options.iter().find(|(dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            grid[ny as usize][nx as usize] != ' '
        });

        if let Some(&(ndx, ndy)) = next_dir {
            dx = ndx;
            dy = ndy;
            x += dx;
            y += dy;
        } else {
            break;
        }
    }

    path_letters.iter().collect()
}

fn p2(input: &str) -> String {
    let grid: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    let mut x: i32 = grid[0].iter().position(|&c| c == '|').unwrap() as i32;
    let mut y: i32 = 0;
    let mut dx: i32 = 0;
    let mut dy: i32 = 1;
    let mut steps = 0;

    loop {
        steps += 1;

        let dir_options = [(dx, dy), (-dy, -dx), (dy, dx)];
        let next_dir = dir_options.iter().find(|(dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            grid[ny as usize][nx as usize] != ' '
        });

        if let Some(&(ndx, ndy)) = next_dir {
            dx = ndx;
            dy = ndy;
            x += dx;
            y += dy;
        } else {
            break;
        }
    }

    steps.to_string()
}

fn main() {
    run_day(p1, p2);
}
