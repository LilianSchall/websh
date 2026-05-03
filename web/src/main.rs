mod components;
mod pages;

use leptos::mount::mount_to_body;
use leptos::prelude::*;
use pages::shell::ShellPage;

fn main() {
    mount_to_body(|| {
        view! {
            <ShellPage />
        }
    });
}
