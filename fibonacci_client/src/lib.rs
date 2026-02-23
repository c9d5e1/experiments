use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::{to_vec, to_vec_pretty};
use tokio::fs::write;
use tokio::spawn;
use tokio::sync::mpsc::channel;
use tokio::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExperimentConfiguration {
    pub label: String,
    pub request_start: usize,
    pub request_increment: usize,
    pub runs: usize,
    pub report_name: String,
    pub target: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExperimentReport {
    configuration: ExperimentConfiguration,
    run_reports: Vec<RunReport>,
    total_requests: usize,
    total_errors: usize,
}

impl ExperimentReport {
    pub fn make_report(configuration: ExperimentConfiguration, run_reports: Vec<RunReport>) -> Self {
        let mut total_requests = 0;
        let mut total_errors = 0;
        for run_report in &run_reports {
            total_requests += run_report.total_requests;
            total_errors += run_report.total_errors;
        }
        Self {
            configuration,
            run_reports,
            total_requests,
            total_errors,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunReport {
    total_requests: usize,
    average_elapsed_millis: u128,
    total_errors: usize,
}

#[derive(Debug)]
pub struct Report {
    now: Instant,
    elapsed: u128,
    result: Result<(), reqwest::Error>,
}

impl Report {
    pub fn new() -> Self {
        Self {
            now: Instant::now(),
            elapsed: 0,
            result: Ok(()),
        }
    }
}

pub async fn get_request(target: &str) -> Result<Vec<u128>, reqwest::Error> {
    reqwest::get(target)
        .await?
        .json::<Vec<u128>>()
        .await
}

pub fn process_reports(reports: Vec<Report>) -> RunReport {
    let len = reports.len();
    if len == 0 {
        panic!("0 reports received")
    }
    let mut full_elapsed_time = 0;
    let mut err_count = 0;
    for report in reports {
        full_elapsed_time += report.elapsed;
        if report.result.is_err() {
            err_count += 1;
            println!("{:#?}", report.result);
        }
    }
    RunReport {
        total_requests: len,
        average_elapsed_millis: full_elapsed_time/len as u128,
        total_errors: err_count,
    }
}

pub async fn run_experiment(conf: ExperimentConfiguration) -> ExperimentReport {

    let mut requests = conf.request_start;
    let mut run_reports = Vec::new();

    for i in 0..conf.runs {

        let (sender, mut receiver) = channel::<Report>(requests);
        for _ in 0..requests {
            let cloned_sender = sender.clone();
            let cloned_target = conf
                .target
                .clone();
            spawn(async move {
                let mut report = Report::new();
                if let Err(err) = get_request(&cloned_target).await {
                    report.result = Err(err);
                }
                report.elapsed = report
                    .now
                    .elapsed()
                    .as_millis();
                let _ = cloned_sender
                    .send(report)
                    .await;
                drop(cloned_sender);
            });
        }
        drop(sender);
        let mut reports = Vec::new();
        loop {
            match receiver.recv().await {
                None =>
                    break,
                Some(report) =>
                    reports.push(report),
            };
        }
        run_reports.push(process_reports(reports));
        requests += conf.request_increment;

        println!("Run {} completed!", i);
    }

    let report_name = conf.report_name.clone();
    let report = ExperimentReport::make_report(conf, run_reports);

    println!("{:#?}", report);
    write(format!("./{}", report_name), to_vec_pretty(&report).unwrap()).await.unwrap();

    report
}