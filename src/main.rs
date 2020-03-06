use slog::{crit,debug, info, error, Drain, Duplicate, Level, LevelFilter};
use slog_async::Async;
use slog_term::{CompactFormat, PlainDecorator};
use slog_json::Json;
use std::fs::OpenOptions;
use std::sync::Mutex;

fn main() {
    let log_path = "test.log";
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_path)
        .unwrap();

    let drain = Mutex::new(Json::default(file)).fuse();
    let file_drain = LevelFilter::new(drain, Level::Info);

    let decorator = PlainDecorator::new(std::io::stderr());
    let err_drain = CompactFormat::new(decorator).build().fuse();


    let drain_pair =Duplicate::new(file_drain, err_drain).fuse();
    let drain = Async::new(drain_pair).build().fuse();


    let log = slog::Logger::root(drain, slog::o!(
                "version" => env!("CARGO_PKG_VERSION"),
                "host" => "localhost",
                "port" => 8080,));

    debug!(log, "started");
    debug!(log, "{} workers", 2;);
    info!(log, "Starting request"; "ip" => "127.0.0.1", "path" => "/", "dur" => 1.2);
    info!(log,  "processing files"; "worker_pool" => 25, "number_of_files" => 12467);
    error!(log, "worker failed"; "worker_id" => 1);
    crit!(log, "server can't continue to work");

}
