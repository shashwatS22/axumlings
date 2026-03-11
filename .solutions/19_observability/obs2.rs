// Manual Spans
//
// Create spans manually when you need fine-grained control over their lifecycle, 
// for example attaching them to specific async blocks rather than entire functions.
//

use tracing::{span, Level, Instrument, info};

pub async fn run_job(job_id: &str) {
    let my_span = span!(Level::INFO, "job_execution", job_id);
    
    let work = async {
        info!("doing work");
    };

    work.instrument(my_span).await;
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
