use std::time::Instant;

pub trait Service: TimeService {}

pub trait TimeService {
    fn monotonic_time(&mut self) -> Instant {
        Instant::now()
    }
}
