use crate::components::App;
use leptos::*;

mod components;
mod error;
mod prelude;

fn main() {
    mount_to_body(|cx| {
        view! {
            cx, <App/>
        }
    })
}
