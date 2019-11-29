#![allow(dead_code)]

use rand::{thread_rng, Rng};
use std::time;

pub fn rand_duration(from_ms: u64, to_ms: u64) -> time::Duration {
    let dur_ms = thread_rng().gen_range(from_ms, to_ms);
    time::Duration::from_millis(dur_ms)
}

pub fn to_millis(duration: time::Duration) -> f64 {
    let sec_ms = duration.as_secs() as f64 * 1e3;
    let subsec_ms = duration.subsec_nanos() as f64 / 1e6;
    sec_ms + subsec_ms
}
