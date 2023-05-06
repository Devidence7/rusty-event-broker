pub trait Event {
    fn name(&self) -> &str;
}
