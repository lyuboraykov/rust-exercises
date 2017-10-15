use std::collections::LinkedList;

struct MetricHolder {
    n: usize,
    sum: u32,
    history: LinkedList<u32>
}

impl MetricHolder {
    fn moving_average(&mut self, new_val: u32) -> f64 {
        self.history.push_back(new_val);
        self.sum += new_val;
        if self.history.len() > self.n {
            let old_val = match self.history.pop_front() {
                Some(val) => val,
                None => 0,
            };
            self.sum -= old_val;
        }

        self.sum as f64 / self.history.len() as f64
    }

    fn new(n: usize) -> MetricHolder {
        MetricHolder {
            n: n,
            sum: 0,
            history: LinkedList::new()
        }
    }
}

fn main() {
    let mut metric_holder = MetricHolder::new(2);
    println!("{}", metric_holder.moving_average(1));
    println!("{}", metric_holder.moving_average(1));
    println!("{}", metric_holder.moving_average(1));
    println!("{}", metric_holder.moving_average(2));
    println!("{}", metric_holder.moving_average(3));
}
