use crate::ActionWithMeta;

/// Function signature for a reducer.
pub type Reducer<State, Action> = fn(&mut State, &ActionWithMeta<Action>);
