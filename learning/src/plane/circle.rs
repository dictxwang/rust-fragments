use crate::action::shape::*;

pub struct Circle {
    radius: f32
}

impl Circle {
    pub fn new(radius: f32) -> Circle {
        Circle { 
            radius: radius
        }
    }
}

impl ShapeAction for Circle {

    fn area(&self) -> f32 {
        return std::f32::consts::PI * self.radius * self.radius;
    }
}