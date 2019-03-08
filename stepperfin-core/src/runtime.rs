use crate::Device;

#[derive(Debug, Clone, PartialEq)]
pub struct Runtime<D> {
    device: D,
}

impl<D: Device> Runtime<D> {
    pub fn new(device: D) -> Runtime<D> {
        Runtime { device }
    }
}
