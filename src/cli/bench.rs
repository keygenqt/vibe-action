//! Benchmark runner.
//! Executes benchmark cases from benchmarks.yaml and prints results.

use clap::Args;
use std::time::Instant;

use crate::{bench::runner::Bench, exit_error, print_info, print_progress, print_success, utils};

#[derive(Args)]
pub struct BenchArgs {
    /// Run benchmarks for a specific action only
    #[arg(long, short = 'a')]
    action: Option<String>,

    /// Show command output
    #[arg(long, short = 'v')]
    verbose: bool,
}

/// Execute all benchmarks.
pub async fn execute(args: BenchArgs) {
    let bench = match Bench::load() {
        Ok(b) => b,
        Err(e) => exit_error!("{}", e),
    };

    let actions: Vec<_> = if let Some(ref filter) = args.action {
        bench
            .config
            .benchmarks
            .iter()
            .filter(|(name, _)| *name == filter)
            .collect()
    } else {
        bench.config.benchmarks.iter().collect()
    };

    if actions.is_empty() {
        print_info!("No benchmark found");
        return;
    }

    let mut passed = 0;
    let mut failed = 0;
    let mut index = 0;
    let total: usize = actions.iter().map(|(_, cases)| cases.len()).sum();
    let start_time = Instant::now();

    print_info!("Start execute task benchmarks ({} cases)", total);

    for (action, cases) in &actions {
        print_info!("Start {}...", action);

        for case in cases.iter() {
            index += 1;

            let start = Instant::now();

            match bench.run(action, case).await {
                Ok(output) => {
                    passed += 1;
                    let duration = start.elapsed();
                    let args_str = case
                        .args
                        .iter()
                        .map(|(_, v)| v.split_whitespace().collect::<Vec<_>>().join(" "))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let args_display = if args_str.chars().count() > 100 {
                        format!("{}...", args_str.chars().take(100).collect::<String>())
                    } else {
                        args_str.clone()
                    };
                    print_progress!(
                        "{:.0}% ({}/{}) | {}",
                        (index as f32 / total as f32) * 100.0,
                        index,
                        total,
                        utils::format::format_duration(duration),
                    );
                    if args.verbose {
                        print_info!("{} {}", action, args_str);
                        if output.is_empty() {
                            print_info!("No matches found.");
                        } else {
                            print_success!("{}", output);
                        }
                    } else {
                        print_progress!("└─ {}", args_display);
                    }
                }
                Err(e) => {
                    failed += 1;
                    let duration = start.elapsed();
                    print_progress!(
                        "└─ {}/{} | {} | FAIL: {}",
                        index,
                        total,
                        utils::format::format_duration(duration),
                        e
                    );
                }
            }
        }
    }

    print_info!(
        "{} passed, {} failed in {}",
        passed,
        failed,
        utils::format::format_duration(start_time.elapsed())
    );
}
