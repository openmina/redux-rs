use std::cell::RefCell;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Sub};
use std::time::Duration;

use crate::SystemTime;

#[derive(Debug, Copy, Clone)]
pub struct Instant {
    inner: wasm_timer::Instant,
}

impl PartialEq for Instant {
    fn eq(&self, other: &Instant) -> bool {
        self.inner == other.inner
    }
}

impl Eq for Instant {}

impl PartialOrd for Instant {
    fn partial_cmp(&self, other: &Instant) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl Ord for Instant {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.partial_cmp(&other.inner).unwrap()
    }
}

thread_local! {
    static INITIAL_AND_DRIFT: RefCell<Option<(SystemTime, wasm_timer::Instant, Duration)>> = RefCell::new(None);
}

impl Instant {
    pub fn now() -> Instant {
        let inner = INITIAL_AND_DRIFT.with_borrow_mut(|initial_and_drift| {
            let (initial_sys_time, initial_monotonic, drift) = initial_and_drift
                .get_or_insert_with(|| {
                    (
                        SystemTime::now(),
                        wasm_timer::Instant::now(),
                        Duration::ZERO,
                    )
                });

            let sys_time_passed = SystemTime::now()
                .duration_since(*initial_sys_time)
                .unwrap_or_default();
            let monotonic_now = wasm_timer::Instant::now();
            let monotonic_passed = monotonic_now.duration_since(*initial_monotonic);

            if sys_time_passed > monotonic_passed + *drift {
                // fix performance.now inaccuracy
                *drift = sys_time_passed - monotonic_passed;
            }

            monotonic_now + *drift
        });

        Self { inner }
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        *self - earlier
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now() - *self
    }

    pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration> {
        match self.cmp(&earlier) {
            Ordering::Less => None,
            _ => Some(self.duration_since(earlier)),
        }
    }

    pub fn saturating_duration_since(&self, earlier: Instant) -> Duration {
        self.checked_duration_since(earlier).unwrap_or_default()
    }

    pub fn checked_add(&self, duration: Duration) -> Option<Instant> {
        Some(*self + duration)
    }

    pub fn checked_sub(&self, duration: Duration) -> Option<Instant> {
        Some(*self - duration)
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, other: Duration) -> Instant {
        Instant {
            inner: self.inner + other,
        }
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, other: Duration) -> Instant {
        Instant {
            inner: self.inner - other,
        }
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Duration {
        self.inner - other.inner
    }
}
