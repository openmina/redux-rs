use crate::{action_queue::ActionQueue, ActionWithMeta, Store};

/// Function signature for a reducer.
pub type Reducer<State, Service, Action> = fn(
    &mut State,
    &ActionWithMeta<Action>,
    &Store<State, Service, Action>,
    &mut ActionQueue<Action, State>,
);
