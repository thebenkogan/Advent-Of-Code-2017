use std::str::FromStr;

use aoc2017::run_day;

struct Firewall {
    layers: Vec<Layer>,
}

impl FromStr for Firewall {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Firewall {
            layers: s
                .split('\n')
                .map(|line| {
                    let (depth, range) = line.split_once(": ").unwrap();
                    Layer::new(depth.parse().unwrap(), range.parse().unwrap())
                })
                .collect(),
        })
    }
}

impl Firewall {
    fn run_packet(&self, t: i32) -> (bool, i32) {
        let mut caught = false;
        let mut severity = 0;
        for layer in &self.layers {
            if layer.scanner_pos(t + layer.depth) == 0 {
                caught = true;
                severity += layer.depth * layer.range;
            }
        }
        (caught, severity)
    }
}

struct Layer {
    depth: i32,
    range: i32,
}

impl Layer {
    fn new(depth: i32, range: i32) -> Self {
        Self { depth, range }
    }

    fn scanner_pos(&self, t: i32) -> i32 {
        let cycle = self.range * 2 - 2;
        let pos = t % cycle;
        if pos < self.range {
            pos
        } else {
            cycle - pos
        }
    }
}

fn p1(input: &str) -> i32 {
    input.parse::<Firewall>().unwrap().run_packet(0).1
}

fn p2(input: &str) -> i32 {
    let firewall = input.parse::<Firewall>().unwrap();
    for delay in 1.. {
        let (caught, _) = firewall.run_packet(delay);
        if !caught {
            return delay;
        }
    }

    unreachable!()
}

fn main() {
    run_day(p1, p2);
}
