#[inline]
pub fn init(max_level: log::Level) {
    fn panick_hook(info: &std::panic::PanicInfo) {
        log::error!("{info}");
    }
    std::panic::set_hook(Box::new(panick_hook));
    wasm_logger::init(wasm_logger::Config::new(max_level));
}
