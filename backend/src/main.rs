mod utils;

fn main() {
    utils::logger::init();

    log_info!("this is an info message");
    log_success!("everything is working");
    log_warn!("this is a warning");
    log_error!("something went wrong");
    log_debug!("debug value: {}", 42);
    log_trace!("trace level message");
}
