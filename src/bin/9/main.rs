use aoc2017::run_day;

fn p1(input: &str) -> i32 {
    let mut level = 0;
    let mut total = 0;
    let mut in_garbage = false;
    let mut ignore = false;

    input.chars().for_each(|c| {
        if ignore {
            ignore = false;
            return;
        }
        if c == '!' {
            ignore = true;
            return;
        }
        if in_garbage {
            if c == '>' {
                in_garbage = false;
            }
            return;
        }
        match c {
            '{' => {
                level += 1;
                total += level;
            }
            '}' => level -= 1,
            '<' => in_garbage = true,
            _ => (),
        }
    });

    total
}

fn p2(input: &str) -> i32 {
    let mut in_garbage = false;
    let mut ignore = false;
    let mut num_chars = 0;

    input.chars().for_each(|c| {
        if ignore {
            ignore = false;
            return;
        }
        if c == '!' {
            ignore = true;
            return;
        }
        if in_garbage {
            if c == '>' {
                in_garbage = false;
            } else {
                num_chars += 1;
            }
            return;
        }
        if c == '<' {
            in_garbage = true;
        }
    });

    num_chars
}

fn main() {
    run_day(p1, p2);
}
