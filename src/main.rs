use crate::app::App;

mod app;
mod db;
mod layout;
mod properties;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
