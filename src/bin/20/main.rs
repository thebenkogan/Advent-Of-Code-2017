use std::collections::HashMap;

use aoc2017::run_day;
use regex::Regex;

#[derive(Debug)]

struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector {
    fn tuple(&self) -> (i64, i64, i64) {
        (self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct Particle {
    id: u32,
    p: Vector,
    v: Vector,
    a: Vector,
}

impl Particle {
    fn step(&mut self) {
        self.v.x += self.a.x;
        self.v.y += self.a.y;
        self.v.z += self.a.z;

        self.p.x += self.v.x;
        self.p.y += self.v.y;
        self.p.z += self.v.z;
    }

    fn distance(&self) -> i64 {
        self.p.x.abs() + self.p.y.abs() + self.p.z.abs()
    }
}

fn parse_particles(input: &str) -> Vec<Particle> {
    let num_regex = Regex::new("-?\\d+").unwrap();
    input
        .split('\n')
        .enumerate()
        .map(|(i, line)| {
            let matches: Vec<i64> = num_regex
                .captures_iter(line)
                .map(|cap| cap[0].parse::<i64>().unwrap())
                .collect();
            Particle {
                id: i as u32,
                p: Vector {
                    x: matches[0],
                    y: matches[1],
                    z: matches[2],
                },
                v: Vector {
                    x: matches[3],
                    y: matches[4],
                    z: matches[5],
                },
                a: Vector {
                    x: matches[6],
                    y: matches[7],
                    z: matches[8],
                },
            }
        })
        .collect()
}

fn p1(input: &str) -> i32 {
    let mut particles = parse_particles(input);

    for _ in 0..1000 {
        for particle in particles.iter_mut() {
            particle.step();
        }
    }

    let closest = particles.iter().min_by_key(|p| p.distance()).unwrap();

    closest.id as i32
}

fn remove_collisions(particles: &mut Vec<Particle>) {
    let mut positions = HashMap::new();

    for particle in particles.iter() {
        let pos = particle.p.tuple();
        *positions.entry(pos).or_insert(0) += 1;
    }

    particles.retain(|particle| {
        let pos = particle.p.tuple();
        positions[&pos] == 1
    });
}

fn p2(input: &str) -> i32 {
    let mut particles = parse_particles(input);

    for _ in 0..1_000 {
        remove_collisions(&mut particles);
        for particle in particles.iter_mut() {
            particle.step();
        }
    }

    particles.len() as i32
}

fn main() {
    run_day(p1, p2);
}
