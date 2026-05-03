use leptos::prelude::*;

stylance::import_style!(style, "pathline.css");

#[component]
pub fn Pathline(
    #[prop(default = "user".to_string())] user: String,
    #[prop(default = "~/".to_string())] cwd: String,
) -> impl IntoView {
    view! {
        <div class=style::pathline>
            <span class=style::user>{user}</span>
            <span class=style::separator>":"</span>
            <span class=style::cwd>{cwd}</span>
        </div>
    }
}
