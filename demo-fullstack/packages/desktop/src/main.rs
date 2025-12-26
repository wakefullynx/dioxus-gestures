use dioxus::prelude::*;

use ui::Example;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Example {}

    }
}
