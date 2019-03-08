/// An operation which can be polled to completion.
pub trait Task<State, Triggers>
where
    State: ?Sized,
    Triggers: ?Sized,
{
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

impl<F, State, Triggers> Task<State, Triggers> for F
where
    F: FnMut(&State, &mut Triggers) -> Continuation,
{
    fn tick(&mut self, state: &State, triggers: &mut Triggers) -> Continuation {
        self(state, triggers)
    }
}

pub fn wait_until<F, State, Triggers>(
    mut predicate: F,
) -> impl Task<State, Triggers>
where
    F: FnMut(&State) -> bool,
{
    move |state: &State, _triggers: &mut Triggers| {
        if predicate(state) {
            Continuation::Complete
        } else {
            Continuation::Continue
        }
    }
}
