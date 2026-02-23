use fibonacci_client::{run_experiment, ExperimentConfiguration};

#[tokio::main]
async fn main() {
    let report = run_experiment(ExperimentConfiguration {
        label: "Performance test medium-laptop".to_string(),
        request_start: 5,
        request_increment: 5,
        runs: 50,
        report_name: "performance_test_medium_laptop.json".to_string(),
        target: "http://192.168.1.137:3000/".to_string(),
    }).await;
}
