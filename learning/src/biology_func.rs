use learning::prelude::animal::*;
use learning::prelude::plant::*;

#[allow(dead_code)]
pub fn biology_show_name<T : PlantAction + AnimalAction>(t: &T) {
    t.show_name();
}

// where语句的使用，使得定义更清晰
#[allow(dead_code)]
pub fn biology_show_name_another<T>(t: &T) 
    where T : PlantAction + AnimalAction {
    t.show_name();
}