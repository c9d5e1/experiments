use fibonacci_client::{run_experiment, ExperimentConfiguration};

#[tokio::main]
async fn main() {
    let report = run_experiment(ExperimentConfiguration {
        label: "Simple performance test bare-pi".to_string(),
        request_start: 2,
        request_increment: 2,
        runs: 50,
        report_name: "simple_performance_test_bare_pi.json".to_string(),
        target: "http://192.168.1.4:3000/".to_string(),
    }).await;
}
