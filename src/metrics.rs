use std::collections::LinkedList;

pub struct MetricHolder {
    n: usize,
    sum: u32,
    history: LinkedList<u32>,
}

impl MetricHolder {
    pub fn moving_average(&mut self, new_val: u32) -> f64 {
        self.history.push_back(new_val);
        self.sum += new_val;
        if self.history.len() > self.n {
            if let Some(val) = self.history.pop_front() {
                self.sum -= val;
            }
        }

        self.sum as f64 / self.history.len() as f64
    }

    pub fn new(n: usize) -> MetricHolder {
        MetricHolder {
            n: n,
            sum: 0,
            history: LinkedList::new(),
        }
    }
}
