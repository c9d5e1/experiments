use fibonacci_client::{run_experiment, ExperimentConfiguration};

#[tokio::main]
async fn main() {
    let report = run_experiment(ExperimentConfiguration {
        label: "Performance test".to_string(),
        request_start: 5,
        request_increment: 5,
        runs: 50,
        report_name: "performance_test.json".to_string(),
        target: "http://127.0.0.1:3000/".to_string(),
    }).await;
}
