#![no_std]

use uom::si::f32::{Length, Velocity};

/// The basic abstraction over device hardware.
pub trait Printer {
    fn axis(&self, letter: char) -> Option<&dyn LinearAxis>;
    fn axis_mut(&self, letter: char) -> Option<&mut dyn LinearAxis>;
}

pub trait LinearAxis {
    /// The current axis position.
    fn current_position(&self) -> Length;
    /// Reset the axis position (e.g. after doing a calibration sequence).
    fn reset_position(&mut self, new_pos: Length);
    /// Set the desired velocity.
    fn set_velocity(&mut self, velocity: Velocity);

    /// Has this axis hit a limit switch (if connected)?
    fn limit_switch_triggered(&self) -> bool {
        false
    }
}
