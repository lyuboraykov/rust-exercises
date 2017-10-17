mod metrics;

fn main() {
    let mut metric_holder = metrics::MetricHolder::new(2);
    println!("{}", metric_holder.moving_average(1));
    println!("{}", metric_holder.moving_average(1));
    println!("{}", metric_holder.moving_average(1));
    println!("{}", metric_holder.moving_average(2));
    println!("{}", metric_holder.moving_average(3));
}
