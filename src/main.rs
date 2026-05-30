use netcalc_rs::components::app::App;

fn main() {
    #[cfg(feature = "desktop")]
    {
        use dioxus::{
            desktop::{Config, WindowBuilder},
            LaunchBuilder,
        };

        let config = Config::new()
            .with_menu(None)
            .with_window(WindowBuilder::new().with_title("NetCalc-rs"));

        LaunchBuilder::desktop().with_cfg(config).launch(App);
    }

    #[cfg(not(feature = "desktop"))]
    {
        dioxus::launch(App);
    }
}
