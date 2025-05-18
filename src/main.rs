use netcalc_rs::components::app::App;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    dioxus::launch(App);
}
