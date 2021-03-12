use std::time::{Duration, Instant};

trait Entity {
    fn update(delta_t: Duration);
}
