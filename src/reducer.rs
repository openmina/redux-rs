use crate::{ActionWithMeta, Dispatcher};

/// Function signature for a reducer.
pub type Reducer<State, Service, Action> =
    fn(&mut State, &ActionWithMeta<Action>, &mut Service, &mut Dispatcher<Action, State>);
