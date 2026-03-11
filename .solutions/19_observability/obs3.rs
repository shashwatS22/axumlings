// Prometheus Metrics
//
// Expose numeric metrics via a `/metrics` endpoint for scraping by Prometheus. 
// 

use metrics::counter;
use metrics_exporter_prometheus::PrometheusBuilder;

pub fn setup_metrics() {
    PrometheusBuilder::new().install_recorder().unwrap();
}

pub fn track_request(method: &str) {
    counter!("http_requests_total", "method" => method.to_string()).increment(1);
}

fn main() {
    // Run `cargo test --bin obs3` to test your solution!
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert!(true);
    }
}
