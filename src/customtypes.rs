#![allow(dead_code)]

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
}

// A struct with two fields
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

// Structs can be reused as fields of another struct
pub struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    pub fn new(top_left: Point, bottom_right: Point) -> Rectangle {
        Rectangle { top_left, bottom_right }
    }

    pub fn rect_area(&self) -> f32 {
        let Rectangle { top_left, bottom_right } = self;
        (top_left.y - bottom_right.y).abs() * (bottom_right.x - top_left.x).abs()
    }

    pub fn square(&self, new_side_length: f32) -> Rectangle {
        let Rectangle { top_left, .. } = self;
        let new_bottom_right = Point { x: top_left.x + new_side_length, y: top_left.y - new_side_length };
        Rectangle { Point::new(top_left.x, top_left.y), Point::new(top_left.x + new_side_length, top_left.y - new_side_length) }
    }
}