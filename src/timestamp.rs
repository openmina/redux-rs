use std::time::Duration;
#[cfg(not(target_arch = "wasm32"))]
pub use std::time::{Instant, SystemTime};
#[cfg(target_arch = "wasm32")]
pub use wasm_timer::{Instant, SystemTime};

use crate::{monotonic_to_time, store::monotonic_time};

/// Time in nanoseconds from [std::time::UNIX_EPOCH].
///
/// Used as an ID+Timestamp for an Action`.
/// Each action will have an unique id. If two actions happen at the same time,
/// id must be increased by 1 for second action, to ensure uniqueness of id.
///
/// u64 is enough to contain time in nanoseconds at most 584 years
/// after `UNIX_EPOCH` (1970-01-01 00:00:00 UTC).
///
/// ```
/// //           nano     micro  milli  sec    min  hour day  year
/// assert_eq!(u64::MAX / 1000 / 1000 / 1000 / 60 / 60 / 24 / 365, 584);
/// ```
#[cfg_attr(feature = "fuzzing", derive(fuzzcheck::DefaultMutator))]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Timestamp(u64);

impl Timestamp {
    pub const ZERO: Self = Self(0);

    #[inline(always)]
    pub fn new(nanos_from_unix_epoch: u64) -> Self {
        Self(nanos_from_unix_epoch)
    }

    #[inline(always)]
    pub fn now() -> Self {
        Timestamp::new(monotonic_to_time(monotonic_time()))
    }

    pub fn checked_sub(self, rhs: Timestamp) -> Option<Duration> {
        self.0.checked_sub(rhs.0).map(Duration::from_nanos)
    }
}

impl From<Timestamp> for u64 {
    fn from(t: Timestamp) -> Self {
        t.0
    }
}

impl std::ops::Add for Timestamp {
    type Output = Timestamp;
    #[inline]
    fn add(self, other: Timestamp) -> Timestamp {
        Timestamp(self.0 + other.0)
    }
}

impl std::ops::Add<u64> for Timestamp {
    type Output = Timestamp;
    #[inline]
    fn add(self, other: u64) -> Timestamp {
        Timestamp(self.0 + other)
    }
}
