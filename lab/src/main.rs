fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::trace!("Trace, world!");
    log::debug!("Debug, world!");
    log::info!("Hello, world!");
    log::warn!("Warning, world!");
    log::error!("Error, world!");
}
