extern crate metrics;

use std::{time, thread};

use metrics::metric_holder::MetricHolder;
use metrics::rate_limiter::{AcquireResult, RateLimiter};


fn main() {
    let mut metric_holder = MetricHolder::new(2);
    let mut moving_average_rate_limiter = RateLimiter::new(1, 10);

    for _ in 0..12 {
        match moving_average_rate_limiter.acquire() {
            AcquireResult::Free => {
                println!("Free to go")
            },
            AcquireResult::Limited => {
                println!("Rate limited, sorry")
            },
        }
    }

    thread::sleep(time::Duration::from_millis(1001));

    for _ in 0..12 {
        match moving_average_rate_limiter.acquire() {
            AcquireResult::Free => {
                println!("Free to go")
            },
            AcquireResult::Limited => {
                println!("Rate limited, sorry")
            },
        }
    }

    println!("{}", metric_holder.moving_average(1));
    println!("{}", metric_holder.moving_average(1));
    println!("{}", metric_holder.moving_average(1));
    println!("{}", metric_holder.moving_average(2));
    println!("{}", metric_holder.moving_average(3));
}
