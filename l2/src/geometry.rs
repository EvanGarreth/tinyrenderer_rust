use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x: x, y: y, z: z }
    }

    pub fn norm(self) -> f64 {
        ((self.x*self.x + self.y*self.y + self.z*self.z) as f64).sqrt()
    }

    pub fn get_x(self) -> i32 {
        self.x
    }
    pub fn get_y(self) -> i32 {
        self.y
    }
    pub fn get_z(self) -> i32 {
        self.z
    }

}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point { 
        Point { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul<Point> for Point {
    type Output = i32;
    fn mul(self, other: Point) -> i32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<i32> for Point {
    type Output = Point;
    fn mul(self, scalar: i32) -> Point {
        Point { x: scalar * self.x, y: scalar * self.y, z: scalar * self.z }
    }
}


