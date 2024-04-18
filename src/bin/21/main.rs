use std::collections::HashMap;

use aoc2017::run_day;

#[derive(Clone)]
struct Image(Vec<Vec<char>>);

impl Image {
    fn new() -> Self {
        let starting_image = vec![
            vec!['.', '#', '.'],
            vec!['.', '.', '#'],
            vec!['#', '#', '#'],
        ];

        Image(starting_image)
    }

    fn symmetric(&self) -> Self {
        let mut new_image = self.0.clone();
        for (i, row) in new_image.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                *col = self.0[j][i];
            }
        }
        Image(new_image)
    }

    fn flip(&self) -> Self {
        let mut new_image = self.0.clone();
        for (i, row) in new_image.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                *col = self.0[self.0.len() - i - 1][j];
            }
        }
        Image(new_image)
    }

    fn all_hashes(&self) -> Vec<String> {
        let mut hashes = vec![];
        let mut image = self.clone();
        for _ in 0..4 {
            hashes.push(image.hash());
            image = image.symmetric();
            hashes.push(image.hash());
            image = image.flip();
        }
        hashes
    }

    fn hash(&self) -> String {
        self.0
            .iter()
            .map(|l| l.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("/")
    }

    fn from_hash(hash: &str) -> Self {
        Image(
            hash.split('/')
                .map(|l| l.chars().collect())
                .collect::<Vec<Vec<char>>>(),
        )
    }

    fn split(&self) -> Vec<Vec<Image>> {
        let mut images = vec![];
        let size = if self.0.len() % 2 == 0 { 2 } else { 3 };
        for i in 0..self.0.len() / size {
            let mut row = vec![];
            for j in 0..self.0.len() / size {
                let mut new_image = vec![];
                for k in 0..size {
                    new_image.push(self.0[i * size + k][j * size..j * size + size].to_vec());
                }
                row.push(Image(new_image));
            }
            images.push(row);
        }
        images
    }

    fn join(images: Vec<Vec<Image>>) -> Self {
        let size = images[0][0].0.len();
        let mut new_image = vec![];
        for row in &images {
            for j in 0..size {
                let mut new_row = vec![];
                for image in row {
                    new_row.extend_from_slice(&image.0[j]);
                }
                new_image.push(new_row);
            }
        }
        Image(new_image)
    }

    fn to_enhanced(&self, enchancement_map: &HashMap<String, String>) -> Self {
        let mapped: Vec<Vec<Image>> = self
            .split()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|image| {
                        for hash in image.all_hashes() {
                            if let Some(new_hash) = enchancement_map.get(&hash) {
                                return Image::from_hash(new_hash);
                            }
                        }
                        unreachable!("No enhancement found for image");
                    })
                    .collect()
            })
            .collect();
        Image::join(mapped)
    }

    fn num_on(&self) -> i32 {
        self.0
            .iter()
            .map(|row| row.iter().filter(|c| **c == '#').count() as i32)
            .sum()
    }
}

fn enchance_times(input: &str, times: i32) -> Image {
    let enchancement_map: HashMap<String, String> =
        HashMap::from_iter(input.split('\n').map(|line| {
            let mut parts = line.split(" => ");
            (
                parts.next().unwrap().to_string(),
                parts.next().unwrap().to_string(),
            )
        }));

    let mut image = Image::new();
    for _ in 0..times {
        image = image.to_enhanced(&enchancement_map);
    }
    image
}

fn p1(input: &str) -> i32 {
    enchance_times(input, 5).num_on()
}

fn p2(input: &str) -> i32 {
    enchance_times(input, 18).num_on()
}

fn main() {
    run_day(p1, p2);
}
