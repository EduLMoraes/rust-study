#[path = "./view/config.rs"]
mod config;

#[path = "./view/homepage.rs"]
mod homepage;

fn main() {
    dioxus_desktop::launch_cfg(
        homepage::app,
        config::config(),
    );
}
