use aoc2017::run_day;

fn p1(input: &str) -> i32 {
    let steps = input.parse::<i32>().unwrap();

    let mut l = Vec::new();
    l.push(0);

    let mut pos: usize = 0;
    for i in 1..2018 {
        pos = (pos + steps as usize) % l.len();
        l.insert(pos + 1, i);
        pos += 1;
    }

    l[(pos + 1) % l.len()]
}

fn p2(input: &str) -> i32 {
    let steps = input.parse::<i32>().unwrap();

    let mut length = 1;
    let mut pos = 0;
    let mut val_after = 0;
    for i in 1..50_000_001 {
        pos = (pos + steps) % length;
        if pos == 0 {
            val_after = i;
        }
        pos += 1;
        length += 1;
    }

    val_after
}

fn main() {
    run_day(p1, p2);
}
