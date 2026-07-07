mod arguments;
mod logging;

use clap::Parser;
use log::{debug, error, info, trace, warn};

use arguments::Args;

fn main() {
    let args: Args = Args::parse();

    logging::_set_log_level(args.verbose);

    info!("{:<10}{}", "URL:", args.url);
    info!("{:<10}{} sec", "Timeout:", args.timeout);
    debug!("{:<10}{}", "Verbose:", args.verbose);
}
