pub trait BiologyAction {
    fn show_name(&self);
}

pub trait BiologyActionSecond<T> {
    fn show_born_info(&self);
}

pub trait BiologyActionThird<T> {
    fn show_born_info(&self);
}