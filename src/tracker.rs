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

    println!("Starting app tracking process...");

    loop {
        let app = get_active_app();

        if let Some(a) = app {
            println!("Detected app: {a}");

            *stats.usage.entry(a).or_insert(0) += 1;
        }

        tick += 1;

        if tick % 60 == 0 {
            let today = get_dates().0;
            let date_str = path_for_date(&today);

            println!("Saving data to `{}`...", date_str.to_str().unwrap());

            create_dir_all(date_str.parent().unwrap()).unwrap();
            write(path_for_date(&today), to_string_pretty(&stats).unwrap()).unwrap();
        }

        sleep(Duration::from_secs(1));
    }
}

pub fn stop_tracking() {
    let pids = get_running_pids();

    for &p in &pids {
        println!("Killing app tracking process... PID: {}", p.to_string());

        let _ = Command::new("kill").arg(p.to_string()).spawn();
    }
}

pub fn get_tracking_status() {
    let pids = get_running_pids();

    if pids.is_empty() {
        println!("Status: Inactive");
    } else {
        println!("Status: Active");
    }
}

fn get_active_app() -> Option<String> {
    if let Ok(info) = get_active_window_info() {
        return Some(info.class_name);
    }

    None
}

fn get_running_pids() -> Vec<u32> {
    let bin_name = current_exe()
        .unwrap_or_default()
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let pid = id();

    if let Ok(output) = Command::new("pgrep").arg("-x").arg(&bin_name).output() {
        let pids = String::from_utf8_lossy(&output.stdout);

        pids.lines()
            .filter_map(|s| s.parse::<u32>().ok())
            .filter(|&p| p != pid)
            .collect()
    } else {
        Vec::new()
    }
}
