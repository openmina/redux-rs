use std::time::Duration;

use crate::{SystemTime, Timestamp};

use super::{ActionId, ActionWithMeta};

pub type RecursionDepth = u32;

/// Action with additional metadata like: id.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActionMeta {
    id: ActionId,
    /// Recursion depth of a given action.
    depth: RecursionDepth,
}

impl ActionMeta {
    #[inline(always)]
    pub(crate) fn new(id: ActionId, depth: RecursionDepth) -> Self {
        Self { id, depth }
    }

    #[inline(always)]
    pub fn id(&self) -> ActionId {
        self.id
    }

    /// Recursion depth of a given action.
    #[inline(always)]
    pub fn depth(&self) -> RecursionDepth {
        self.depth
    }

    #[inline(always)]
    pub fn time(&self) -> Timestamp {
        self.id.into()
    }

    #[inline(always)]
    pub fn sys_time(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + self.duration_since_epoch()
    }

    #[inline(always)]
    pub fn time_as_nanos(&self) -> u64 {
        self.id.into()
    }

    #[inline(always)]
    pub fn duration_since_epoch(&self) -> Duration {
        Duration::from_nanos(self.time_as_nanos())
    }

    #[inline(always)]
    pub fn duration_since(&self, other: &ActionMeta) -> Duration {
        self.id.duration_since(other.id)
    }

    #[inline(always)]
    pub fn with_action<T>(self, action: T) -> ActionWithMeta<T> {
        ActionWithMeta { meta: self, action }
    }
}
