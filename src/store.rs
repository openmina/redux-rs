use std::sync::OnceLock;

use crate::{
    ActionId, ActionMeta, ActionWithMeta, Effects, EnablingCondition, Instant, Reducer, SystemTime,
    TimeService, SubStore,
};

/// Wraps around State and allows only immutable borrow,
/// Through `StateWrapper::get` method.
///
/// Mutable borrow of state can only happen in reducer.
pub struct StateWrapper<State> {
    inner: State,
}

impl<State> StateWrapper<State> {
    /// Get immutable reference to State.
    #[inline(always)]
    pub fn get(&self) -> &State {
        &self.inner
    }

    /// Get mutable reference to State.
    ///
    /// Only should be used in the reducer and it's not `pub`,
    /// so it can't be accessed from lib users.
    #[inline(always)]
    fn get_mut(&mut self) -> &mut State {
        &mut self.inner
    }
}

impl<T: Clone> Clone for StateWrapper<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

/// Monotonic and system time reference points.
static INITIAL_TIME: OnceLock<(Instant, SystemTime)> = OnceLock::new();

/// Converts monotonic time to nanoseconds since Unix epoch.
pub fn monotonic_to_time(time: Instant) -> u64 {
    let (monotonic, system) = INITIAL_TIME.get_or_init(|| (Instant::now(), SystemTime::now()));
    let time_passed = time.duration_since(*monotonic);
    system
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|x| x + time_passed)
        .map(|x| x.as_nanos())
        .unwrap_or(0) as u64
}

/// Main struct for the state machine.
///
/// Exposes a [Store::dispatch](redux::Store::dispatch) method, using
/// which [Action](redux::Action) can be dispatched, which triggers a
/// 1. [Reducer](redux::Reducer) - to update the state.
/// 2. [Effects](redux::Effects) - to trigger side-effects of the action.
pub struct Store<State, Service, Action> {
    reducer: Reducer<State, Action>,
    effects: Effects<State, Service, Action>,

    /// Current State.
    ///
    /// Immutable access can be gained using `store.state.get()`.
    /// Mutation can only happen inside reducer.
    pub state: StateWrapper<State>,
    pub service: Service,

    monotonic_time: Instant,

    /// Current recursion depth of dispatch.
    recursion_depth: u32,

    last_action_id: ActionId,
}

impl<State, Service, Action> Store<State, Service, Action>
where
    Service: TimeService,
{
    /// Creates a new store.
    pub fn new(
        reducer: Reducer<State, Action>,
        effects: Effects<State, Service, Action>,
        mut service: Service,
        initial_time: SystemTime,
        initial_state: State,
    ) -> Self {
        let initial_monotonic_time = service.monotonic_time();
        let initial_time_nanos = initial_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|x| x.as_nanos())
            .unwrap_or(0);

        INITIAL_TIME.get_or_init(move || (initial_monotonic_time, initial_time));

        Self {
            reducer,
            effects,
            service,
            state: StateWrapper {
                inner: initial_state,
            },

            monotonic_time: initial_monotonic_time,

            recursion_depth: 0,
            last_action_id: ActionId::new_unchecked(initial_time_nanos as u64),
        }
    }

    /// Returns the current state.
    #[inline(always)]
    pub fn state(&self) -> &State {
        self.state.get()
    }

    #[inline(always)]
    pub fn service(&mut self) -> &mut Service {
        &mut self.service
    }

    /// Convert monotonic time to system clock in nanoseconds from epoch.
    pub fn monotonic_to_time(&self, monotonic_time: Instant) -> u64 {
        monotonic_to_time(monotonic_time)
    }

    /// Dispatch an Action.
    ///
    /// Returns `true` if the action was enabled, hence if it was dispatched
    /// to reducer and then effects.
    ///
    /// If action is not enabled, we return false and do nothing.
    pub fn dispatch<T>(&mut self, action: T) -> bool
    where
        T: Into<Action> + EnablingCondition<State>,
    {
        if !action.is_enabled(self.state(), self.last_action_id.into()) {
            return false;
        }
        self.dispatch_enabled(action.into());

        true
    }

    /// Dispatch an Action (For `SubStore`).
    ///
    /// Returns `true` if the action was enabled, hence if it was dispatched
    /// to reducer and then effects.
    ///
    /// If action is not enabled, we return false and do nothing.
    pub fn sub_dispatch<A, S>(&mut self, action: A) -> bool
    where
        A: Into<<Self as SubStore<State, S>>::SubAction> + EnablingCondition<S>,
        <Self as SubStore<State, S>>::SubAction: Into<Action>,
        Self: SubStore<State, S>,
    {
        if !action.is_enabled(<Self as SubStore<State, S>>::state(self), self.last_action_id.into()) {
            return false;
        }
        self.dispatch_enabled(action.into().into());

        true
    }

    /// Dispatches action without checking the enabling condition.
    fn dispatch_enabled(&mut self, action: Action) {
        let monotonic_time = self.service.monotonic_time();
        let time_passed = monotonic_time
            .duration_since(self.monotonic_time)
            .as_nanos();
        let prev = self.last_action_id;
        self.last_action_id = prev.next(time_passed as u64);
        self.recursion_depth += 1;

        let action_with_meta =
            ActionMeta::new(self.last_action_id, prev, self.recursion_depth).with_action(action);

        self.dispatch_reducer(&action_with_meta);
        self.dispatch_effects(action_with_meta);

        self.recursion_depth -= 1;
    }

    /// Runs the reducer.
    #[inline(always)]
    fn dispatch_reducer(&mut self, action_with_id: &ActionWithMeta<Action>) {
        (self.reducer)(self.state.get_mut(), action_with_id);
    }

    /// Runs the effects.
    #[inline(always)]
    fn dispatch_effects(&mut self, action_with_id: ActionWithMeta<Action>) {
        (self.effects)(self, action_with_id);
    }
}

impl<State, Service, Action> Clone for Store<State, Service, Action>
where
    State: Clone,
    Service: Clone,
    Action: Clone,
{
    fn clone(&self) -> Self {
        Self {
            reducer: self.reducer,
            effects: self.effects,
            service: self.service.clone(),
            state: self.state.clone(),

            monotonic_time: self.monotonic_time,

            recursion_depth: self.recursion_depth,
            last_action_id: self.last_action_id,
        }
    }
}
