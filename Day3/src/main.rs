use std::fs;
//use std::cmp::Ordering;

#[derive(Debug)]
struct Wire {
    points: Vec<Point>,
}

impl Wire {
    fn add_point(&mut self, point: &Point) {
        self.points.push(*point);
    }

    fn point_exists(&mut self, point: &Point) -> bool {

        let point_found = match self.points.binary_search(&point) {
            Ok(_pos) => true,
            Err(_pos) => false
        };

        point_found
    }

    fn add_points(&mut self, dir: char, distance: i32) {
        let last_point = match self.points.last() {
            Some(point) => point.clone(),
            _ => Point{x: 0, y: 0}
        };

        for amount in 1..=distance {
            let new_point = match dir {
                'U' => Point{x: last_point.x, y: last_point.y + amount},
                'D' => Point{x: last_point.x, y: last_point.y - amount},
                'L' => Point{x: last_point.x - amount, y: last_point.y },
                'R' => Point{x: last_point.x + amount, y: last_point.y },
                _ => panic!()
            };

            self.add_point(&new_point);
        }
    }

    fn intersect(&mut self, other: &Wire) -> Vec<Point> {
        let mut out = vec![];
        self.points.sort_unstable();
        for x in other.points.iter() {
            if self.point_exists(x) {
                out.push(*x)
            }
        }

        out
      }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn distance_to_center(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let wire_instructions: Vec<Vec<_>> = contents.lines().map(|wire| wire.split(",").collect()).collect();

    let mut wire1 = Wire {
        points: Vec::new()
    };

    let mut wire2 = Wire {
        points: Vec::new()
    };

    for (i, instructions) in wire_instructions.iter().enumerate() {
        let wire: &mut Wire;

        if i == 0 {
            wire = &mut wire1;
        } else {
            wire = &mut wire2;
        }

        for instruction in instructions {

            let dir = match &instruction.chars().next() {
                Some(my_char) => *my_char,
                _ => 'U'
            };

            let distance = match &instruction[1..].parse::<i32>() {
                Ok(int) => *int,
                _ => 0
            };

            wire.add_points(dir, distance)
        }
    }

    let mut intersect = wire1.intersect(&wire2);

    intersect.sort_by(|a,b| a.distance_to_center().cmp(&b.distance_to_center()));

    println!("Manhatten distance to closest intersection: {}", intersect.first().unwrap().distance_to_center());
}
