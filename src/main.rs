extern crate rand;

const N: usize = 3;

struct MetricHolder {
    // TODO: make N a part of the metric holder and use a vector
    // also improve by persisting the sum instead of doing avg every time
    history: [u32; N],
    counter: usize
}

impl MetricHolder {
    fn moving_average(&mut self, new_val: u32) -> f64 {
        self.history[self.counter] = new_val;
        self.counter = (self.counter + 1) % N;
        let mut sum: u32 = 0;
        let mut count: u32 = 0;
        for &el in self.history.iter() {
            sum += el;
            if el != 0 {
                count += 1;
            }
        }
        sum as f64 / count as f64
    }
}

fn main() {

    // TODO: implement default
    let mut metric_holder = MetricHolder {
        history: [0; N],
        counter: 0
    };

    println!("{}", metric_holder.moving_average(1));
    println!("{}", metric_holder.moving_average(1));
    println!("{}", metric_holder.moving_average(1));
    println!("{}", metric_holder.moving_average(2));
    println!("{}", metric_holder.moving_average(3));
}
