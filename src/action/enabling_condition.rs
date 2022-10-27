pub trait EnablingCondition<State> {
    /// Enabling condition for the Action.
    ///
    /// Checks if the given action is enabled for a given state.
    fn is_enabled(&self, #[allow(unused_variables)] state: &State) -> bool {
        true
    }
}
