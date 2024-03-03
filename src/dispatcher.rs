use std::{collections::VecDeque, marker::PhantomData};

pub struct Dispatcher<Action, State> {
    queue: VecDeque<Action>,
    _marker: PhantomData<State>,
}

impl<Action, State> Dispatcher<Action, State>
where
    Action: crate::EnablingCondition<State>,
{
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            _marker: Default::default(),
        }
    }

    pub fn push<T>(&mut self, action: T)
    where
        T: Into<Action>,
    {
        self.queue.push_back(action.into());
    }

    pub(crate) fn pop(&mut self) -> Option<Action> {
        self.queue.pop_front()
    }

    pub(crate) fn push_front(&mut self, other: Dispatcher<Action, State>) {
        other
            .queue
            .into_iter()
            .rev()
            .for_each(|action| self.queue.push_front(action));
    }
}
