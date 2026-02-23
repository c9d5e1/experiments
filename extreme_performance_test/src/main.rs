use fibonacci_client::{run_experiment, ExperimentConfiguration};

#[tokio::main]
async fn main() {
    let report = run_experiment(ExperimentConfiguration {
        label: "Extreme performance test".to_string(),
        request_start: 10,
        request_increment: 10,
        runs: 40,
        report_name: "extreme_performance_test.json".to_string(),
        target: "http://127.0.0.1:3000/".to_string(),
    }).await;
}
