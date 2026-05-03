use leptos::prelude::*;

stylance::import_style!(style, "promptline.css");

#[component]
pub fn Promptline(
    #[prop(default = "❯".to_string())] symbol: String,
    #[prop(default = "".to_string())] command_preview: String,
) -> impl IntoView {
    view! {
        <div class=style::promptline>
            <span class=style::symbol>{symbol}</span>
            <span class=style::input>{command_preview}</span>
        </div>
    }
}
