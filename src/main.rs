mod cli;
mod storage;
mod tracker;
mod tui;

use clap::Parser;
use cli::Args;
use std::{
    env::current_exe,
    process::{Command, id},
};

fn main() {
    let bin_name = current_exe()
        .unwrap_or_default()
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let args = Args::parse();

    if args.start {
        tracker::run_tracking();
    } else if args.stop {
        let pid = id();

        if let Ok(output) = Command::new("pgrep").arg("-x").arg(&bin_name).output() {
            let pids = String::from_utf8_lossy(&output.stdout);

            for pid_str in pids.lines() {
                if let Ok(p) = pid_str.parse::<u32>() {
                    if p != pid {
                        let _ = Command::new("kill").arg(p.to_string()).status();
                    }
                }
            }
        }
    } else if args.status {
        // TODO
    } else if args.tui {
        tui::run_tui();
    }
}
