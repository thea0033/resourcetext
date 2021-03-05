use Shape::Intersect;

use crate::location::Location;

pub enum Shape {
    Circle(f64, Location),
    Intersect(Vec<Shape>),
    Not(Box<Shape>),
    Any(Vec<Shape>),
}
impl Shape {
    pub fn is_inside(&self, pt: Location) -> bool {
        match self {
            Shape::Circle(val1, val2) => val2.close_enough(&pt, *val1),
            Intersect(val) => {
                for line in val {
                    if !line.is_inside(pt) {
                        return false;
                    }
                }
                true
            }
            Shape::Any(val) => {
                for line in val {
                    if line.is_inside(pt) {
                        return true;
                    }
                }
                false
            }
            Shape::Not(val) => !(*val).is_inside(pt),
        }
    }
    pub fn habitable_zone(star_location: Location, intensity: f64) -> Shape {
        let shape1 = Shape::Circle(intensity * 2.0, star_location);
        let shape2 = Shape::Not(Box::new(Shape::Circle(intensity, star_location)));
        let vector = vec![shape1, shape2];
        Intersect(vector)
    }
}
