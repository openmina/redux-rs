use crate::EnablingCondition;
use std::collections::VecDeque;
use std::marker::PhantomData;

pub struct ActionQueue<Action, State> {
    queue: VecDeque<AnyAction<Action, State>>,
    _marker: PhantomData<(Action, State)>,
}

impl<Action, State> ActionQueue<Action, State> {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            _marker: PhantomData,
        }
    }

    pub fn push<T>(&mut self, action: T)
    where
        T: 'static + EnablingCondition<State> + Into<Action> + Clone + Send,
        State: 'static,
        Action: 'static,
    {
        let wrapped_action = AnyAction::<Action, State>::new(action);
        self.queue.push_back(wrapped_action);
    }

    pub(crate) fn pop(&mut self) -> Option<AnyAction<Action, State>> {
        self.queue.pop_front()
    }

    pub(crate) fn push_front(&mut self, other: ActionQueue<Action, State>) {
        other
            .queue
            .into_iter()
            .rev()
            .for_each(|action| self.queue.push_front(action));
    }

    pub fn sub_queue<SubAction, SubState>(&self) -> ActionQueue<SubAction, SubState>
    where
        SubAction: Into<Action>,
    {
        ActionQueue::new()
    }
}

trait ActionConvertible<A, S>: EnablingCondition<S> {
    fn convert(&self) -> A;
}

pub(crate) struct AnyAction<A, S> {
    inner: Box<dyn ActionConvertible<A, S> + Send>,
}

impl<A, S> AnyAction<A, S> {
    fn new<T>(action: T) -> Self
    where
        T: 'static + ActionConvertible<A, S> + Send,
    {
        Self {
            inner: Box::new(action),
        }
    }

    pub fn is_enabled(&self, state: &S) -> bool {
        self.inner.is_enabled(state)
    }

    pub fn convert(&self) -> A {
        self.inner.convert()
    }
}

impl<T, A, S> ActionConvertible<A, S> for T
where
    T: 'static + Clone + Into<A> + EnablingCondition<S>,
{
    fn convert(&self) -> A {
        self.clone().into()
    }
}
