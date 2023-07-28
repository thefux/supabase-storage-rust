use super::{builder::Builder, executor::Executor};

impl Builder {
    pub fn delete_object(self) -> Executor {
        self.create_executor()
    }
}
