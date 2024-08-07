pub mod app;
pub mod pages;

use app::*;
use leptos::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    logging::log!("101");

    mount_to_body(|| {
        view! { <App /> }
    });
}
