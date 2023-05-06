pub trait Request {
    fn name(&self) -> &str;
}

pub trait Response {
    fn name(&self) -> &str;

    // TODO: Change this
    fn data(&self) -> &str;
}
