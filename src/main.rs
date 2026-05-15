use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        h1 { font_size: "50px", "i love eating sand!!!" }
    }
}
