pub trait Build {
    type Output;
    fn execute(self) -> Self::Output;
}
