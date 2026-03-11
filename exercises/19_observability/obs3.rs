// Prometheus Metrics
//
// Expose numeric metrics via a `/metrics` endpoint for scraping by Prometheus. 
// 
// I AM NOT DONE

use metrics::counter;
use metrics_exporter_prometheus::PrometheusBuilder;

pub fn setup_metrics() {
    // TODO: Create a PrometheusBuilder and install the recorder
    // Hint: `PrometheusBuilder::new().install_recorder().unwrap();`
    todo!()
}

pub fn track_request(method: &str) {
    // TODO: Increment a counter named `http_requests_total` with the dimension `method`
    // Hint: `counter!("http_requests_total", "method" => method.to_string()).increment(1);`
    todo!()
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
