use leptos::*;
mod error;
mod prelude;

fn main() {
    mount_to_body(|cx| {
        view! {
            cx, <p>"Hello, world!"</p>
        }
    })
}
