use std::{
    env,
    fmt::Display,
    fs,
    path::Path,
    time::{Duration, Instant},
};

pub fn run_day<T: Display>(p1: fn(&str) -> T, p2: fn(&str) -> T) {
    let (input, bench) = get_input();
    if bench {
        benchmark_parts(p1, p2, &input)
    } else {
        run_parts(p1, p2, &input)
    };
}

fn get_input() -> (String, bool) {
    let mut args = env::args();
    let path = args.next().unwrap();
    let flag = args.next().unwrap_or("".to_string());
    let day = Path::new(&path).file_name().unwrap().to_str().unwrap();
    let input_file = if flag == "test" { "test" } else { "in" };
    let path = format!("src/bin/{day}/{input_file}.txt");
    let input = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("file {path} not found"))
        .to_string();
    (input, flag == "bench")
}

fn run_parts<T: Display>(p1: fn(&str) -> T, p2: fn(&str) -> T, input: &str) {
    println!("Part 1: {}", p1(input));
    println!("Part 2: {}", p2(input));
}

const NUM_TRIALS: u32 = 100;

fn benchmark_parts<T>(p1: fn(&str) -> T, p2: fn(&str) -> T, input: &str) {
    println!("Part 1 Time: {:?}", benchmark_part(p1, input));
    println!("Part 2 Time: {:?}", benchmark_part(p2, input));
}

fn benchmark_part<T>(p: fn(&str) -> T, input: &str) -> Duration {
    let mut total = Duration::ZERO;
    for _ in 0..NUM_TRIALS {
        let start = Instant::now();
        p(input);
        let elapsed = start.elapsed();
        total += elapsed
    }
    total / NUM_TRIALS
}

pub const DIAG_DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
