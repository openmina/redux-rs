use crate::{Instant, store::monotonic_time};

pub trait Service: TimeService {}

/// Time service.
pub trait TimeService {
    /// NOTE this should be deprecated as monotonic time now is inherited from Rust library.
    fn monotonic_time(&mut self) -> Instant {
        monotonic_time()
    }
}
