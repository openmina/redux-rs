use std::sync::OnceLock;

use crate::{
    action_queue::ActionQueue, ActionId, ActionMeta, ActionWithMeta, Effects, EnablingCondition,
    Instant, Reducer, SystemTime, TimeService,
};

/// Wraps around State and allows only immutable borrow,
/// Through `StateWrapper::get` method.
///
/// Mutable borrow of state can only happen in reducer.
pub struct StateWrapper<State> {
    inner: Option<State>,
}

impl<State> Default for StateWrapper<State> {
    fn default() -> Self {
        Self { inner: None }
    }
}

impl<State> StateWrapper<State> {
    /// Get immutable reference to State.
    #[inline(always)]
    pub fn get(&self) -> &State {
        self.inner.as_ref().unwrap()
    }

    /// Get mutable reference to State.
    ///
    /// Only should be used in the reducer and it's not `pub`,
    /// so it can't be accessed from lib users.
    #[inline(always)]
    fn get_mut(&mut self) -> &mut State {
        self.inner.as_mut().unwrap()
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
    reducer: Reducer<State, Service, Action>,
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

    /// Queue for actions to be dispatched after the state update
    dispatch_queue: ActionQueue<Action, State>, // TODO
}

impl<State, Service, Action> Store<State, Service, Action>
where
    Service: TimeService,
{
    /// Creates a new store.
    pub fn new(
        reducer: Reducer<State, Service, Action>,
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
                inner: Some(initial_state),
            },

            monotonic_time: initial_monotonic_time,

            recursion_depth: 0,
            last_action_id: ActionId::new_unchecked(initial_time_nanos as u64),

            dispatch_queue: ActionQueue::new(),
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
        if !action.is_enabled(self.state()) {
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
    pub fn sub_dispatch<SubAction, A>(&mut self, action: A) -> bool
    where
        A: Into<SubAction> + EnablingCondition<State>,
        SubAction: Into<Action>,
    {
        if !action.is_enabled(self.state()) {
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

        self.monotonic_time = monotonic_time;
        self.last_action_id = self.last_action_id.next(time_passed as u64);

        let action_with_meta =
            ActionMeta::new(self.last_action_id, self.recursion_depth).with_action(action);
        self.recursion_depth += 1;

        // TODO: instead return queued actinos, pass them to dispatch_effects
        self.dispatch_reducer(&action_with_meta);
        self.dispatch_effects(action_with_meta);

        self.recursion_depth -= 1;
    }

    /// Runs the reducer.
    #[inline(always)]
    fn dispatch_reducer(&mut self, action_with_id: &ActionWithMeta<Action>) {
        // All new queued elements will be stored here
        let mut queue = ActionQueue::new();
        let mut state = std::mem::take(&mut self.state);
        (self.reducer)(state.get_mut(), action_with_id, self, &mut queue);
        self.state = state;
        // All the enqueued actions gets pushed to the front of the global queue
        self.dispatch_queue.push_front(queue);
    }

    /// Runs the effects.
    #[inline(always)]
    fn dispatch_effects(&mut self, action_with_id: ActionWithMeta<Action>) {
        // Dispatch all enqueued actions
        while let Some(action) = self.dispatch_queue.pop() {
            if action.is_enabled(self.state()) {
                self.dispatch_enabled(action.convert());
            }
        }
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

            dispatch_queue: ActionQueue::new(), // TODO: clone
        }
    }
}
