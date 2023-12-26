use color_eyre::eyre::Context;
use time::macros::format_description;
use time::UtcOffset;
use tracing::metadata::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

pub fn setup() -> Result<(), color_eyre::Report> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let offset = UtcOffset::current_local_offset()
        .with_context(|| "failed to get current timezone offset")?;
    let timer = OffsetTime::new(offset, format_description!("[hour]:[minute]:[second]"));

    let fmt_layer = tracing_subscriber::fmt::layer().with_timer(timer);

    let fmt_layer_filtered = fmt_layer.with_filter(env_filter);

    tracing_subscriber::registry()
        // add the console layer to the subscriber
        // .with(console_subscriber::spawn())
        .with(ErrorLayer::default())
        .with(fmt_layer_filtered)
        // set the registry as the default subscriber
        .init();

    color_eyre::install()?;

    Ok(())
}
