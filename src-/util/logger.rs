
pub fn init_logger() {
    env_logger::Builder::new()
        .format_timestamp(None)
        .init();
}