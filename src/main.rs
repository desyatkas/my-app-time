mod cli;
mod storage;
mod tracker;
mod tui;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    if args.start {
        tracker::start_tracking();
    } else if args.stop {
        tracker::stop_tracking();
    } else if args.status {
        // TODO
    } else if args.tui {
        tui::run_tui();
    }
}
