mod prelude;
use crate::prelude::*;

fn main() {
    dioxus_desktop::launch_cfg(
        homepage::app,
        config::config(),
    );
}
