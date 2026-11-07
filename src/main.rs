mod cli;
mod storage;
mod tracker;
mod tui;

use clap::Parser;
use cli::Args;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.start {
        tracker::start_tracking().await;
    } else if args.stop {
        tracker::stop_tracking();
    } else if args.status {
        tracker::get_tracking_status();
    } else if args.tui {
        tui::run_tui();
    }
}
