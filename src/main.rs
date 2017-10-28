extern crate service_utils;

use service_utils::metric_aggregator::MetricAggregator;


fn main() {
    let mut aggregator = MetricAggregator::new(10);
    aggregator.report_metric(4.0);
    aggregator.report_metric(2.0);
    aggregator.report_metric(3.0);
    aggregator.report_metric(7.0);
    aggregator.report_metric(1.0);
    aggregator.report_metric(6.0);
    aggregator.report_metric(5.0);
    aggregator.report_metric(8.0);
    aggregator.report_metric(9.0);
    aggregator.report_metric(10.0);
    println!("{}", aggregator.percentile(95));
    println!("{}", aggregator.mean());
}
