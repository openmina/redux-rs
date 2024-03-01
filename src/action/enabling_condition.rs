#[allow(unused_variables)]
pub trait EnablingCondition<State> {
    /// Enabling condition for the Action.
    ///
    /// Checks if the given action is enabled for a given state.
    fn is_enabled(&self, state: &State) -> bool {
        true
    }

    /// Enabling condition for the Action.
    ///
    /// Checks if the given action is enabled for a given state.
    fn is_enabled_with_time(&self, state: &State, time: crate::Timestamp) -> bool {
        self.is_enabled(state)
    }
}

