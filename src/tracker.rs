use crate::storage::{get_dates, load_for_date, path_for_date};
use kdotool::get_active_window_info;
use serde_json::to_string_pretty;
use std::{
    env::current_exe,
    fs::{create_dir_all, write},
    process::{Command, id},
    thread::sleep,
    time::Duration,
};

pub fn start_tracking() {
    let mut tick = 0;
    let mut stats = load_for_date(&get_dates().0);

    loop {
        let app = get_active_app();

        if let Some(a) = app {
            *stats.usage.entry(a).or_insert(0) += 1;
        }

        tick += 1;

        if tick % 60 == 0 {
            let today = get_dates().0;

            create_dir_all(path_for_date(&today).parent().unwrap()).unwrap();
            write(path_for_date(&today), to_string_pretty(&stats).unwrap()).unwrap();
        }

        sleep(Duration::from_secs(1));
    }
}

pub fn stop_tracking() {
    let bin_name = current_exe()
        .unwrap_or_default()
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
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
}

pub fn get_tracking_status() {
    let bin_name = current_exe()
        .unwrap_or_default()
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let pid = id();

    if let Ok(output) = Command::new("pgrep").arg("-x").arg(&bin_name).output() {
        let mut running = false;
        let pids = String::from_utf8_lossy(&output.stdout);

        for pid_str in pids.lines() {
            if let Ok(p) = pid_str.parse::<u32>() {
                if p != pid {
                    running = true;

                    break;
                }
            }
        }

        if running {
            println!("Running");
        } else {
            println!("Not running");
        }
    }
}

pub fn get_active_app() -> Option<String> {
    if let Ok(info) = get_active_window_info() {
        return Some(info.class_name);
    }

    None
}
