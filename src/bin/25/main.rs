use std::collections::HashMap;

use aoc2017::run_day;

fn p1(_: &str) -> i32 {
    // hard coding this because I am lazy
    let states = HashMap::from([
        ('A', vec![(1, 1, 'B'), (0, -1, 'C')]),
        ('B', vec![(1, -1, 'A'), (1, 1, 'D')]),
        ('C', vec![(1, 1, 'A'), (0, -1, 'E')]),
        ('D', vec![(1, 1, 'A'), (0, 1, 'B')]),
        ('E', vec![(1, -1, 'F'), (1, -1, 'C')]),
        ('F', vec![(1, 1, 'D'), (1, 1, 'A')]),
    ]);
    let steps = 12919244;
    let mut state = 'A';

    let mut cursor = 0;
    let mut tape = HashMap::new();

    for _ in 0..steps {
        let current = tape.entry(cursor).or_insert(0);
        let (write, move_cursor, new_state) = states.get(&state).unwrap()[*current as usize];
        *current = write;
        cursor += move_cursor;
        state = new_state;
    }

    tape.values().sum()
}

fn p2(_: &str) -> i32 {
    0
}

fn main() {
    run_day(p1, p2);
}
