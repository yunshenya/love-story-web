use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init(){
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| tracing_subscriber::EnvFilter::from("info")))
        .with(tracing_subscriber::fmt::layer()
            .with_ansi(true)
            .with_file(true)
            .with_line_number(true)
            .with_target(false)
            .with_level(true)
            .with_thread_names(true)
        )
    .init();
}