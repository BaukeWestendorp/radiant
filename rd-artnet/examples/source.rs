use std::{thread, time::Duration};

fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_builder().filter_level(log::LevelFilter::Debug).init();

    let _source = rd_artnet::Source::new(rd_artnet::NetworkConfig::Default {
        interface_name: Some("en0".to_string()),
    })?;

    thread::sleep(Duration::from_secs(3600));

    Ok(())
}
