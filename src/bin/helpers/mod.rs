#![allow(dead_code)]

use std::{thread, time};

pub fn sleep(dur_ms: u64) {
    thread::sleep(time::Duration::from_millis(dur_ms));
}

pub fn to_millis(duration: time::Duration) -> f64 {
    let sec_ms = duration.as_secs() as f64 * 1e3;
    let subsec_ms = duration.subsec_nanos() as f64 / 1e6;
    sec_ms + subsec_ms
}
