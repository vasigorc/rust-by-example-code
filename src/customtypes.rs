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
        Rectangle { top_left: Point::new(top_left.x, top_left.y), bottom_right: new_bottom_right }
    }
}

mod tests {
    use rstest::{rstest, fixture};
    use crate::{Rectangle, Point};

    #[fixture]
    fn fixture() -> Rectangle {
        Rectangle::new(Point::new(1.0, 5.0), Point::new(7.0, 10.0))
    }

    #[rstest]
    fn rect_area_calculated_for_rectangle_of_30(fixture: Rectangle) {
        assert_eq!(fixture.rect_area(), 30f32)
    }

    #[rstest]
    fn square_of_5(fixture: Rectangle) {
        let side_length = 5f32;
        let Rectangle { top_left, bottom_right } = fixture.square(side_length);
        let height = top_left.y - bottom_right.y;
        let width = bottom_right.x - top_left.x;
        assert_eq!(height, side_length);
        assert_eq!(width, side_length)
    }
}
