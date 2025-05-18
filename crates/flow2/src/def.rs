use std::any::Any;
use std::fmt::Debug;

pub trait Node {
    fn id(&self) -> &'static str;

    fn process(&self, inputs: Vec<Box<dyn ValueType>>) -> Vec<Box<dyn ValueType>>;
}

pub trait ValueType: Debug {
    fn as_any(&self) -> &dyn Any;

    fn clone_box(&self) -> Box<dyn ValueType>;
}

impl<T: 'static + Clone + Debug> ValueType for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn ValueType> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn ValueType> {
    fn clone(&self) -> Box<dyn ValueType> {
        self.clone_box()
    }
}
