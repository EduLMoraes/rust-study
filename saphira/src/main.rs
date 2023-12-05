mod prelude;
use prelude::*;
fn main() {
    dioxus_desktop::launch_cfg(
        homepage::app,
        config::config(),
    );
}
