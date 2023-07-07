use std::{process, time::Duration};

use job_scheduler::{Job, JobScheduler, Schedule};

use crate::config::Config;

mod config;
mod ddns;

fn main() {
    let config = match Config::from_toml() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    println!(
        "=========\ndo-dns started\nSchedule: {}\nDNS Record: {}.{}\n=========",
        &config.schedule, &config.record_name, &config.domain
    );

    let mut scheduler = JobScheduler::new();
    let schedule: Schedule = match config.schedule.parse() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    scheduler.add(Job::new(schedule, ddns::from_config(config)));

    loop {
        scheduler.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
}
