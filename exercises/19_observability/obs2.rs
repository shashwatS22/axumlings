// Manual Spans
//
// Create spans manually when you need fine-grained control over their lifecycle,
// for example attaching them to specific async blocks rather than entire functions.
//
// I AM NOT DONE

use tracing::{info, span, Instrument, Level};

pub async fn run_job(job_id: &str) {
    // TODO: Create a span named "job_execution" with `job_id` as a field using `span!`
    // Hint: `let my_span = span!(Level::INFO, "job_execution", job_id);`

    let work = async {
        info!("doing work");
    };

    // TODO: attach the span to the `work` future using `.instrument(my_span)` and await it
    // Hint: `work.instrument(my_span).await;`
}

fn main() {
    // Run `cargo test --bin obs2` to test your solution!
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert!(true);
    }
}
