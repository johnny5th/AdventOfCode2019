use std::fs;
use std::cmp::Ordering;

#[derive(Debug)]
struct Wire {
    points: Vec<Point>,
}

impl Wire {
    fn add_point(&mut self, point: &Point) {
        self.points.push(*point);
    }

    fn point_exists(&self, point: &Point) -> Result<usize, usize> {
        self.points.binary_search(&point)
    }

    fn add_points(&mut self, dir: char, distance: i32) {
        // Grab the last point to buid off of before sorting.
        let last_point = match self.points.last() {
            Some(point) => point.clone(),
            _ => Point{x: 0, y: 0, steps: 0}
        };

        // Sort the points for binary searching
        self.points.sort_unstable();

        for amount in 1..=distance {
            let new_point = match dir {
                'U' => Point{x: last_point.x, y: last_point.y + amount, steps: last_point.steps + amount},
                'D' => Point{x: last_point.x, y: last_point.y - amount, steps: last_point.steps + amount},
                'L' => Point{x: last_point.x - amount, y: last_point.y, steps: last_point.steps + amount},
                'R' => Point{x: last_point.x + amount, y: last_point.y, steps: last_point.steps + amount},
                _ => panic!()
            };

            // Assume we can binary search and not worry about our newly added points being a duplicate.
            match self.point_exists(&new_point) {
                Err(_num) => self.add_point(&new_point),
                _ => {}
            }
        }
    }

    fn intersect(&mut self, other: &Wire) -> Vec<[Point; 2]> {
        let mut out = vec![];
        self.points.sort_unstable();
        for x in other.points.iter() {
            match self.point_exists(x) {
                Ok(_num) => out.push([self.points[_num], *x]),
                _ => {}
            }
        }
        out
    }
}

#[derive(Debug, Clone, Copy, Eq)]
struct Point {
    x: i32,
    y: i32,
    steps: i32
}

impl Point {
    fn distance_to_center(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x, self.y).cmp(&(other.x, other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

    let intersect = wire1.intersect(&wire2);

    let mut intersect_closest_to_center = intersect.clone();
    intersect_closest_to_center.sort_by(|a,b| a[0].distance_to_center().cmp(&b[0].distance_to_center()));
    let intersect_closest_to_center = intersect_closest_to_center.first().unwrap()[0].distance_to_center();

    let mut intersect_fewest_steps = intersect.clone();
    intersect_fewest_steps.sort_by(|a,b| (a[0].steps + a[1].steps).cmp(&(b[0].steps + b[1].steps)));
    let intersect_fewest_steps = intersect_fewest_steps.first().unwrap();
    let intersect_fewest_steps = {
        intersect_fewest_steps[0].steps + intersect_fewest_steps[1].steps
    };

    println!("Intersection closest to origin: {}", intersect_closest_to_center);

    println!("Intersection with fewest steps: {}", intersect_fewest_steps);
}
