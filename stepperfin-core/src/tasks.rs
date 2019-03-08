/// An arbitrary operation which can be periodically polled to completion.
pub trait Task<State, Triggers> {
    /// Make progress on the task.
    fn tick(&mut self, state: &State, triggers: &mut Triggers) -> Continuation;
}

/// What should be done after polling a `Task`?
pub enum Continuation {
    /// Continue polling the task.
    Continue,
    /// The task has reached a failed end state.
    Faulted,
    /// The task has run to completion.
    Complete,
}
