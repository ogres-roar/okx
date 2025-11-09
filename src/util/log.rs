pub fn init_log() {
    log4rs::init_file("conf/log.yml", Default::default()).unwrap();
}