#[macro_use]
mod logger;

fn main() {
    log_fatal!("Fatal");
    log_error!("Error");
    log_warn!("warn");
    log_info!("info");
    log_debug!("debug");
    log_trace!("trace");
}
