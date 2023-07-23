use super::biology::BiologyAction;  // 引用同一个module下的特性

// 特性的继承
pub trait AnimalAction : BiologyAction {
    fn do_jump(&self);
}