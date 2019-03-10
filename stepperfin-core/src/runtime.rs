use crate::{Clock, Device};

#[derive(Debug, Clone, PartialEq)]
pub struct Runtime<D> {
    device: D,
}

impl<D: Device> Runtime<D> {
    pub fn new(device: D) -> Runtime<D> {
        Runtime { device }
    }

    pub fn step(&mut self, clock: &dyn Clock) {
        self.fill_command_buffer();
        self.update_motion_plan();
        self.update_motion();
        self.check_limits();

        if let Some(display) = self.device.display_mut() {
            display.tick(clock);
        }
    }

    fn fill_command_buffer(&mut self) {
        // copy any newly received data into our gcode buffer
    }

    fn update_motion_plan(&mut self) {
        // parse any new gcodes and add them to the motion plan
    }

    fn update_motion(&mut self) {
        // make sure the device is executing the current motion command
    }

    fn check_limits(&mut self) {
        // check any limit switches and maybe abort the current recipe
    }
}
