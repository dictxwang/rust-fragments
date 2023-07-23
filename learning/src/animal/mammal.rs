use crate::action::{
    animal::AnimalAction,
    biology::BiologyAction,
    biology::BiologyActionSecond,
    biology::BiologyActionThird,
    component::Head,
    component::Roof
};

#[derive(Debug, Clone, Copy)]
pub struct Dog<T> {
    color: T,
    name: T
}

impl Dog<String> {
    pub fn new(name: String, color: String) -> Dog<String> {
        Dog {
            color: color,
            name: name
        }
    }
}


// AnimalAction 继承了BiologyAction，所以需要对Dog结构体分别实现这两个特性
impl AnimalAction for Dog<String> {

    fn do_jump(&self) {
        println!("The dog with {} skin is jumpping.", self.color);
    }
}

impl BiologyAction for Dog<String> {

    fn show_name(&self) {
        println!("The dog's name is {}.", self.name);
    }
}

pub struct BasicInfo {
    pub size: f32
}

// trait的泛型是struct时，不能使用多个
impl BiologyActionSecond<BasicInfo> for Dog<String> {
    
    fn show_born_info(&self) {}
}

impl <P: Head + Roof> BiologyActionSecond<P> for Dog<String> {
    fn show_born_info(&self) {
        println!("Its born info is {}", self.name);
    }
}

// 使用where语句，可以让泛型类型限定更清晰
impl <P> BiologyActionThird<P> for Dog<String>
    where P : Head + Roof {
    fn show_born_info(&self) {
    }
}