// use crate::action;


pub struct Cube {
    length: f32,
    width: f32,
    height: f32,
}

impl Cube {
    pub fn new(length: f32, width: f32, height: f32) -> Cube {
        Cube {
            length: length,
            width: width,
            height: height
        }
    }

    // &slef 属于不可变的地址传参
    pub fn volume(&self) -> f32 {
        return self.length * self.width * self.height;
    }
}