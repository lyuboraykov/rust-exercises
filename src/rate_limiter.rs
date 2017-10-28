use std::collections::LinkedList;
use std::time::{SystemTime, UNIX_EPOCH};
use std::cmp::Ordering;

pub enum AcquireResult {
    Limited,
    Free,
}

pub struct RateLimiter {
    call_times: LinkedList<u64>,
    window_size: u64, // seconds
    limit: u32
}

impl RateLimiter {
    pub fn acquire(&mut self) -> AcquireResult {
        let now_time = SystemTime::now();
        let now_seconds = now_time.duration_since(UNIX_EPOCH)
                                  .expect("Failed to get duration since epoch")
                                  .as_secs();
        self.call_times.push_back(now_seconds);
        let mut redundant_calls_count = 0;
        for call_time in self.call_times.iter() {
            if now_seconds - call_time >= self.window_size {
                redundant_calls_count += 1;
            } else {
                break;
            }
        }
        for _ in 0..redundant_calls_count {
            self.call_times.pop_front();
        }
        return match self.call_times.len().cmp(&(self.limit as usize)) {
            Ordering::Greater => AcquireResult::Limited,
            _ => AcquireResult::Free
        };
    }

    pub fn new(window_size: u64, limit: u32) -> RateLimiter {
        return RateLimiter {
            window_size: window_size,
            limit: limit,
            call_times: LinkedList::new()
        }
    }
}
