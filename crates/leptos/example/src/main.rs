use crate::app::App;

mod app;

pub fn main() {
    console_log::init_with_level(log::Level::Debug).expect("Console logger should be available");
    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
