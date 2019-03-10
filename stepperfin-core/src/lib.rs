#![no_std]

pub mod runtime;
pub mod tasks;

pub use crate::runtime::Runtime;

use core::time::Duration;
use uom::si::f32::{Length, Velocity};

/// The basic abstraction over device hardware.
pub trait Device {
    fn axis(&self, letter: char) -> Option<&dyn LinearAxis>;
    fn axis_mut(&self, letter: char) -> Option<&mut dyn LinearAxis>;

    fn display(&self) -> Option<&dyn Display> {
        None
    }
    fn display_mut(&mut self) -> Option<&mut dyn Display> {
        None
    }
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

pub trait Clock {
    fn time_since_start(&self) -> Duration;
}

/// An LCD display.
pub trait Display {
    /// Let the [`Display`] do any necessary updates (e.g. for animations).
    fn tick(&mut self, clock: &dyn Clock);
}
