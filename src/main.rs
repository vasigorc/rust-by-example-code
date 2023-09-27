use crate::customtypes::{Point, Rectangle};

mod primitives;
mod customtypes;

fn main() {
    let person = Rectangle::new(Point::new(1.0, 5.0), Point::new(7.0, 10.0));
    assert_eq!(person.rect_area(), 30.0)
}
