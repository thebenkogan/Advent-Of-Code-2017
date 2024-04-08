use aoc2017::{knot_hash::knot_hash, run_day, DIRS};

fn build_grid(key: &str) -> Vec<Vec<bool>> {
    (0..128)
        .map(|num| {
            let hash = knot_hash(&format!("{}-{}", key, num));
            hash.chars()
                .map(|c| c.to_digit(16).unwrap())
                .flat_map(|n| {
                    (0..4)
                        .map(|i| (n >> i) & 1 == 1)
                        .rev()
                        .collect::<Vec<bool>>()
                })
                .collect()
        })
        .collect()
}

fn p1(key: &str) -> i32 {
    build_grid(key).iter().flatten().filter(|&&bit| bit).count() as i32
}

fn p2(key: &str) -> i32 {
    let grid = build_grid(key);

    let mut seen = [[false; 128]; 128];
    let mut num_regions = 0;
    for y in 0..128 {
        for x in 0..128 {
            if grid[y][x] && !seen[y][x] {
                let mut stack = vec![(x, y)];
                seen[y][x] = true;
                num_regions += 1;
                while let Some((x, y)) = stack.pop() {
                    for (dx, dy) in DIRS {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if (0..128).contains(&nx)
                            && (0..128).contains(&ny)
                            && grid[ny as usize][nx as usize]
                            && !seen[ny as usize][nx as usize]
                        {
                            stack.push((nx as usize, ny as usize));
                            seen[ny as usize][nx as usize] = true;
                        }
                    }
                }
            }
        }
    }

    num_regions
}

fn main() {
    run_day(p1, p2);
}
