use std::any::Any;
use std::fmt::Debug;

pub trait Component: Send + Sync + Debug {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any + Send + Sync + Debug> Component for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait NamedComponent {
    const NAME: &'static str;
}
