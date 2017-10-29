use std::collections::LinkedList;
use std::cmp::Ordering::Equal;
use std::fmt;

pub struct MetricAggregator {
    n: usize,
    sum: f64,
    history: LinkedList<f64>,
}

impl MetricAggregator {
    pub fn new(n: usize) -> MetricAggregator {
        MetricAggregator {
            n: n,
            sum: 0.0,
            history: LinkedList::new(),
        }
    }

    pub fn report_metric(&mut self, metric: f64) {
        self.history.push_back(metric);
        self.sum += metric;
        if self.history.len() > self.n {
            if let Some(val) = self.history.pop_front() {
                self.sum -= val;
            }
        }
    }

    pub fn mean(&self) -> f64 {
        self.sum / self.history.len() as f64
    }

    pub fn percentile(&self, p: u32) -> f64 {
        let mut sorted_history: Vec<_> = self.history.iter().collect();
        sorted_history.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
        let index = (p as f64 / 100f64) * self.history.len() as f64;
        // do floor because indexes start from 0
        *sorted_history[index.floor() as usize]
    }
}

impl fmt::Display for MetricAggregator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MetricAggregator with bucket size - {}", self.n)
    }
}
